// What the Before step says about a recipe: the words for the shared
// BeforePage (../kit/before.h), taken only from what `recipe` reported. It says
// nothing about how much is downloaded or how much disk is used, because
// `recipe` does not report either.
#pragma once

#include "before.h"
#include "client.h"
#include "model.h"

// What is about to be done: an install into one of `choices`, or an update
// where the software is (`place`, the only choice).
struct Ask {
    QString name;
    bool update = false;
    Place place = Place::Computer;
    QList<Place> choices;
};

// The key a place has on the page, and back.
QString placeKey(Place place);
Place placeFromKey(const QString &key);

BeforePage::Content beforeContent(const Catalog &catalog, const Places &places, const Ask &ask, Place chosen);

// "host (port 443)" and the list of them; shared with the recipe page.
QString endpointText(const QString &endpoint);
QString endpointsText(const QStringList &endpoints);
