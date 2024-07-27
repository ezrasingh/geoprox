export * from './geohashApiApi';
import { GeohashApiApi } from './geohashApiApi';
export * from './geoshardApiApi';
import { GeoshardApiApi } from './geoshardApiApi';
import * as http from 'http';

export class HttpError extends Error {
    constructor (public response: http.IncomingMessage, public body: any, public statusCode?: number) {
        super('HTTP request failed');
        this.name = 'HttpError';
    }
}

export { RequestFile } from '../model/models';

export const APIS = [GeohashApiApi, GeoshardApiApi];
