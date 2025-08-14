import ChessBoard from "@/components/mainView/ChessBoard";
import LastMovesSideBar from "@/components/mainView/LastMovesSideBar";

function GameView() {
  return (
    <div style={{ display: "flex" }}>
      <div style={{ width: "80%" }}>
        <ChessBoard firstMove="white" />
      </div>
      <div style={{ width: "20%" }}>
        <LastMovesSideBar />
      </div>
    </div>
  );
}

export default GameView;
