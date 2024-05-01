import { useState } from "react";
import { Marker } from "react-leaflet/Marker";
import { Popup } from "react-leaflet/Popup";
import { useMapEvents } from "react-leaflet/hooks";
import { LatLng, LatLngExpression } from "leaflet";
import { markerIcon } from "../helpers";
import { AddRider, AddOrder, Search } from "./steps";

enum UiAction {
  DEFAULT,
  ADD_RIDER,
  ADD_ORDER,
  SEARCH,
}

export const MapDialog = () => {
  const [step, setStep] = useState(UiAction.DEFAULT);
  const [position, setPosition] = useState<LatLngExpression>();
  const map = useMapEvents({
    click({ latlng }) {
      setStep(UiAction.DEFAULT);
      setPosition(latlng);
      map.flyTo(latlng, map.getZoom());
    },
  });
  if (!position) return <></>;
  const closeDialog = () => setPosition(undefined);
  const uiSwitch = {
    [UiAction.DEFAULT]: (
      <>
        <button
          className="btn btn-md btn-primary mb-2"
          onClick={(e) => {
            e.stopPropagation();
            setStep(UiAction.ADD_RIDER);
          }}
        >
          Place New Rider
        </button>
        <button
          className="btn btn-md btn-primary my-2"
          onClick={(e) => {
            e.stopPropagation();
            setStep(UiAction.ADD_ORDER);
          }}
        >
          Place New Order
        </button>
        <button
          className="btn btn-md btn-primary mt-2"
          onClick={(e) => {
            e.stopPropagation();
            setStep(UiAction.SEARCH);
          }}
        >
          Search within radius
        </button>
      </>
    ),
    [UiAction.ADD_RIDER]: (
      <AddRider coord={position as LatLng} closeDialog={closeDialog} />
    ),
    [UiAction.ADD_ORDER]: (
      <AddOrder coord={position as LatLng} closeDialog={closeDialog} />
    ),
    [UiAction.SEARCH]: <Search />,
  };
  return (
    <Marker position={position} icon={markerIcon}>
      <Popup>
        <div className="flex flex-col justify-center p-4">{uiSwitch[step]}</div>
      </Popup>
    </Marker>
  );
};
