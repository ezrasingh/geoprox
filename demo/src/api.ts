import axios from "axios";
import { LatLng } from "leaflet";

const api = axios.create({
    baseURL: "/",
    withCredentials: true,
});

export async function placeRider(uid: number, position: LatLng){
    const { lat, lng } = position.wrap();
    return await api.post('/rider/', { uid, position: [lat, lng] });
}

export async function removeRider(uid: number){
    return await api.delete('/rider/', { data: { uid } });
}

export async function placeOrder(distance: number, position: LatLng){
    const { lat, lng } = position.wrap();
    return await api.post('/order/', { distance, location: [lat, lng] });
}