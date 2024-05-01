import { useState } from "react";
import { LatLng } from "leaflet";
import { useAppState } from "../../store";

export interface AddOrderProps {
  coord: LatLng;
  closeDialog: () => void;
}

export const AddOrder: React.FC<AddOrderProps> = ({ coord, closeDialog }) => {
  const placeOrder = useAppState((state) => state.placeOrder);
  const [name, setName] = useState("");
  return (
    <form
      className="mx-auto"
      onSubmit={(e) => {
        e.preventDefault();
        placeOrder({ name, position: coord });
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
          placeholder="Customer name"
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
