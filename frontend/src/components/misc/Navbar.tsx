import Settings from "@/views/Settings";
import { Button } from "@/components/ui/button";

interface Props {
  startGame: (value: boolean) => void;
}

function Navbar({ startGame }: Props) {
  return (
    <div
      className="navbar"
      style={{
        display: "flex",
        justifyContent: "flex-end",
        backgroundColor: "#b6b2a1",
        width: "100%",
        padding: "0 16px",
      }}
    >
      <Button className="mt-2 mb-2 mr-2" onClick={() => startGame(true)}>
        Start
      </Button>

      <Settings />
    </div>
  );
}

export default Navbar;
