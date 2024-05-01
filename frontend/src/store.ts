import { create } from "zustand";
import { immer } from "zustand/middleware/immer";
import { enableMapSet } from "immer";
import { LatLng, LatLngExpression } from "leaflet";
import * as api from './api'; 

enableMapSet();

interface User{
    uid: number;
    name: string;
    position: LatLngExpression;
}

export interface AppState {
  riders: Map<string, User>
  orders: Map<string, User>;
}

export interface AppActions {
  placeRider: (rider: User) => Promise<void>;
  removeRider: (name: string) => void;
  placeOrder: (order: User) => void;
  cancelOrder: (name: string) => void;
}

let id = 0;

export const useAppState = create(
    immer<AppState & AppActions>(
        (set, get) => ({
            riders: new Map(),
            orders: new Map(),
            async placeRider(rider) {
                const uid = id++;
                await api.placeRider(uid, rider.position as LatLng);
                set((state) => {
                    state.riders.set(rider.name, { ...rider, uid });
                });
            },
            async removeRider(name){
                const rider = get().riders.get(name);
                if(rider){
                    await api.removeRider(rider.uid);
                    set((state) => {
                        state.riders.delete(name);
                    })
                }
            },
            placeOrder(order) {
                set((state) => {
                    state.orders.set(order.name, order);
                });
            },
            cancelOrder(name) {
                set((state) => {
                    state.orders.delete(name)
                })
            },
        })
    )
);