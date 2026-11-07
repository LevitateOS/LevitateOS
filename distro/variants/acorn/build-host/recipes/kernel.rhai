// Build-host contract declaration: Acorn kernel install must go through Recipe Rhai.
// Published runtime bundles are validated by the prebuilt artifact recipe.
let required_kernel_recipe = "distro/builder/recipes/linux-prebuilt.rhai";
let required_invocation = "recipe install";
