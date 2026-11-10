// `--status`: which agent is chosen and whether it can be used.
#pragma once

#include "settings.h"

#include <QJsonObject>

QJsonObject agentStatus(const Settings &settings);
