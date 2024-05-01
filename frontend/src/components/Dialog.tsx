import { useState } from "react";
import cx from "classnames";
import { useAppState } from "../store";
type DialogOption = "riders" | "orders";

export const Dialog = () => {
  const { riders, orders, cancelOrder } = useAppState((state) => ({
    riders: Array.from(state.riders.values()),
    orders: Array.from(state.orders.values()),
    cancelOrder: state.cancelOrder,
  }));
  const [open, setOpen] = useState<DialogOption>("riders");
  const users = open === "riders" ? riders : orders;
  return (
    <div className="absolute z-50 w-96 h-96 bg-white top-4 left-16 shadow-lg rounded-md overflow-auto">
      <div
        role="tablist"
        className="tabs tabs-boxed bg-white sticky top-0 py-2 z-30 shadow-md"
      >
        {["riders", "orders"].map((option) => (
          <a
            key={option}
            role="tab"
            className={cx("tab", "capitalize", {
              "tab-active": open === option,
            })}
            onClick={() => setOpen(option as DialogOption)}
          >
            {option}
          </a>
        ))}
      </div>
      <div className="flex flex-col p-8 overflow-auto h-full relative">
        {users.map((u, index) => (
          <div
            key={index}
            className="flex justify-between items-center outline outline-1 hover:outline-2 rounded-sm my-2 py-2 px-2 select-none"
          >
            <span className="text-black font-bold">{u.name}</span>
            <span>
              <button
                className="btn btn-sm btn-error"
                onClick={() => cancelOrder(u.name)}
              >
                delete
              </button>
            </span>
          </div>
        ))}
      </div>
    </div>
  );
};
