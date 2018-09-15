import * as express from 'express';
import * as bodyParser from 'body-parser';

export const main = express();
main.post('/', bodyParser.json(), function(req, res) {
    if (typeof req.body !== 'object') {
        res.status(400).send('body is not a JSON object');
        return;
    }
    const x = req.body.x;
    const y = req.body.y;
    if (typeof x !== 'number' || typeof y !== 'number') {
        res.status(400).send('fields x and y must be numbers');
        return;
    }
    res.status(200).send(JSON.stringify({ result: x + y }));
});