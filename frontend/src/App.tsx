import "./App.css";
import GameView from "./views/GameView";
import NavBar from "./components/misc/Navbar";
import { useState } from "react";

function App() {
  const [startGame, setStartGame] = useState(false);

  return (
    <>
      <NavBar startGame={setStartGame} />
      <GameView startGame={startGame} />
    </>
  );
}

export default App;
