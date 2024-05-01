import { MapContainer } from "react-leaflet/MapContainer";
import { TileLayer } from "react-leaflet/TileLayer";
import { LatLngExpression } from "leaflet";
import "leaflet/dist/leaflet.css";
import { useAppState } from "../store";
import { Marker, Popup } from "react-leaflet";
import { markerIcon, orderIcon, riderIcon } from "../helpers";

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
            <div className="bg-white m-8 p-4">{user.name}</div>
          </Popup>
        </Marker>
      ))}
      {riders.map((user, id) => (
        <Marker key={id} icon={riderIcon} position={user.position}>
          <Popup>
            <div className="bg-white m-8 p-4">{user.name}</div>
          </Popup>
        </Marker>
      ))}
    </MapContainer>
  );
};
