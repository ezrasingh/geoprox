import { MapContainer } from "react-leaflet/MapContainer";
import { TileLayer } from "react-leaflet/TileLayer";
import { LatLng, LatLngExpression } from "leaflet";
import "leaflet/dist/leaflet.css";
import { useAppState } from "../store";
import { Marker, Popup } from "react-leaflet";
import { orderIcon, riderIcon } from "../helpers";
import * as api from "../api";

const CENTER: LatLngExpression = {
  lat: 40.77638178482896,
  lng: -433.97163391113287,
};

export const WorldMap: React.FC<React.PropsWithChildren> = ({ children }) => {
  const { orders, riders } = useAppState((state) => ({
    orders: Array.from(state.orders.values()),
    riders: Array.from(state.riders.values()),
  }));
  return (
    <MapContainer
      style={{ zIndex: 0 }}
      className="overflow-hidden h-screen w-screen"
      center={CENTER}
      zoom={15}
      scrollWheelZoom
    >
      <TileLayer
        attribution='&copy; <a href="https://www.openstreetmap.org/copyright">OpenStreetMap</a> contributors'
        url="https://{s}.tile.openstreetmap.org/{z}/{x}/{y}.png"
      />
      {children}
      {orders.map((user, id) => (
        <Marker key={id} icon={orderIcon} position={user.position}>
          <Popup>
            <div className="bg-white m-8 p-4">
              <b className="font-bold">Order:</b>&nbsp;<span>{user.name}</span>
            </div>
            <button
              className="btn btn-primary btn-wide mx-auto"
              onClick={async () => {
                const radius = prompt("Enter search radius", "0.0");
                if (radius === null) return;
                const { data } = await api.placeOrder(
                  parseFloat(radius),
                  user.position as LatLng
                );
                alert(`Found ${JSON.stringify(data, null, 1)}`);
              }}
            >
              Search nearby
            </button>
          </Popup>
        </Marker>
      ))}
      {riders.map((user, id) => (
        <Marker key={id} icon={riderIcon} position={user.position}>
          <Popup>
            <div className="bg-white m-8 p-4">
              <b className="font-bold">Rider:</b>&nbsp;
              <span>
                {user.name} (uid={user.uid})
              </span>
            </div>
          </Popup>
        </Marker>
      ))}
    </MapContainer>
  );
};
