import { WorldMap, MapDialog, Dialog } from "./components";

const App = () => {
  return (
    <main>
      <Dialog />
      <WorldMap>
        <MapDialog />
      </WorldMap>
    </main>
  );
};

export default App;
