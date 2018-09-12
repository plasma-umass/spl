
/**
 *
 * @param {Object} req A Javascript object of the JSON body, or null if there is no parsable JSON body
 * @param {Object} res A Javascript object of the GET query.
 */
function plotjson(jsonBody, getQuery) {
    return `Hello ${jsonBody.name || 'World'}!`;
}

exports.plotjson = plotjson;