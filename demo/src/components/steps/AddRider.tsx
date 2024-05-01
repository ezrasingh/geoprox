import { useState } from "react";
import { LatLng } from "leaflet";
import { useAppState } from "../../store";

export interface AddRiderProps {
  coord: LatLng;
  closeDialog: () => void;
}

export const AddRider: React.FC<AddRiderProps> = ({ coord, closeDialog }) => {
  const placeRider = useAppState((state) => state.placeRider);
  const [name, setName] = useState("");
  return (
    <form
      className="mx-auto"
      onSubmit={(e) => {
        e.preventDefault();
        placeRider({ name, position: coord });
        closeDialog();
      }}
    >
      <label className="input input-bordered bg-white flex items-center gap-2">
        Name
        <input
          id="name"
          name="name"
          type="text"
          className="grow"
          placeholder="Rider name"
          value={name}
          onChange={(e) => setName(e.target.value)}
        />
      </label>
      <button className="btn btn-primary w-full capitalize mt-4">
        place order
      </button>
    </form>
  );
};
