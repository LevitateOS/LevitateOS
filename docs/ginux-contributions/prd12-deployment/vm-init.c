#define _GNU_SOURCE
#include <errno.h>
#include <fcntl.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <sys/mount.h>
#include <sys/reboot.h>
#include <sys/stat.h>
#include <sys/utsname.h>
#include <sys/wait.h>
#include <sys/xattr.h>
#include <unistd.h>

static void finish(int success, const char *reason) {
    printf("PRD12_VM %s %s errno=%d\n", success ? "PASS" : "FAIL", reason, success ? 0 : errno);
    fflush(stdout);
    sync();
    reboot(RB_POWER_OFF);
    for (;;) pause();
}
#define CHECK(condition, reason) do { if (!(condition)) finish(0, reason); } while (0)
static void run(char *const argv[]) {
    pid_t child = fork();
    CHECK(child >= 0, "fork");
    if (child == 0) { execv(argv[0], argv); _exit(127); }
    int status;
    CHECK(waitpid(child, &status, 0) == child, "wait");
    printf("PRD12_VM command=%s status=%d\n", argv[0], status);
    CHECK(WIFEXITED(status) && WEXITSTATUS(status) == 0, "child failed");
}
static struct stat metadata(const char *path, unsigned mode) {
    struct stat result;
    CHECK(lstat(path, &result) == 0, "missing deployed inode");
    CHECK((result.st_mode & 07777) == mode && result.st_uid == 74 && result.st_gid == 81,
          "target mode/uid/gid");
    CHECK(result.st_mtim.tv_sec == 0 && result.st_mtim.tv_nsec == 0, "normalized image timestamp");
    return result;
}
int main(void) {
    /* Refuse to act outside this dedicated disposable guest. */
    if (getpid() != 1) return 125;
    mkdir("/proc", 0755);
    if (mount("proc", "/proc", "proc", 0, NULL)) return 124;
    char command_line[4096];
    int fd = open("/proc/cmdline", O_RDONLY);
    ssize_t size = fd < 0 ? -1 : read(fd, command_line, sizeof command_line - 1);
    if (fd >= 0) close(fd);
    if (size <= 0) return 123;
    command_line[size] = 0;
    if (!strstr(command_line, "prd12_deployment_fixture=1")) return 122;
    fd = open("/dev/console", O_RDWR);
    if (fd >= 0) { dup2(fd, 0); dup2(fd, 1); dup2(fd, 2); if (fd > 2) close(fd); }
    setvbuf(stdout, NULL, _IONBF, 0);
    CHECK(mount("devtmpfs", "/dev", "devtmpfs", 0, NULL) == 0, "devtmpfs");
    CHECK(mount("sysfs", "/sys", "sysfs", 0, NULL) == 0, "sysfs");
    setenv("PATH", "/usr/bin:/bin", 1);
    setenv("LC_ALL", "C", 1);
    struct utsname kernel;
    CHECK(uname(&kernel) == 0, "uname");
    printf("PRD12_VM BEGIN kernel=%s network=none target=disposable-tmpfs\n", kernel.release);
    run((char *[]){"/usr/bin/sha256sum", "/recstrap", "/payload/target.erofs", NULL});
    CHECK(mount("tmpfs", "/target", "tmpfs", 0, "size=3G") == 0, "target tmpfs");
    /* No force, host-root-filesystem or other installer guard waiver. */
    run((char *[]){"/recstrap", "/target", "--rootfs", "/payload/target.erofs", "--json", NULL});
    struct stat helper = metadata("/target/usr/bin/helper", 04750);
    struct stat alias = metadata("/target/usr/bin/alias", 04750);
    CHECK(helper.st_dev == alias.st_dev && helper.st_ino == alias.st_ino, "hardlink identity");
    metadata("/target/usr/bin/group-helper", 02755);
    metadata("/target/usr/bin/through-alias", 05111);
    metadata("/target/tmp", 01777);
    metadata("/target/bin", 0777);
    char data[64] = {0};
    fd = open("/target/usr/bin/helper", O_RDONLY);
    CHECK(fd >= 0 && read(fd, data, sizeof data) == 15 && !memcmp(data, "target payload\n", 15), "payload bytes");
    CHECK(close(fd) == 0, "payload close");
    memset(data, 0, sizeof data);
    CHECK(readlink("/target/bin", data, sizeof data) == 8 && !memcmp(data, "/usr/bin", 8), "absolute target symlink");
    const unsigned char capability[20] = {1, 0, 0, 2, 0, 32};
    unsigned char actual[64];
    CHECK(getxattr("/target/usr/bin/helper", "security.capability", actual, sizeof actual) == 20 &&
          !memcmp(actual, capability, sizeof capability), "capability bytes");
    CHECK(umount("/target") == 0, "target teardown");
    finish(1, "actual-recstrap=1 zstd-erofs=1 uid-gid=1 setuid=1 setgid=1 sticky=1 links=1 capabilities=1 epoch-mtime=1");
}
