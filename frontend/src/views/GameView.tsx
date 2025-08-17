import ChessBoard from "@/components/mainView/ChessBoard";
import LastMovesSideBar from "@/components/mainView/LastMovesSideBar";
import api from "@/services/api";
import type { InitTypes, WebSocketData } from "@/types/RequestTypes";
import { useEffect, useState, useRef } from "react";

function GameView() {
  const hasLoaded = useRef(false);
  const [firstColorMove, setFirstColorMove] = useState("white");

  const addFieldColorToPosition = (
    position: Array<Array<string>>
  ): Array<Array<{ field: string; figureType: string }>> | null => {
    let fieldColor = firstColorMove;

    const newPosition: Array<Array<{ field: string; figureType: string }>> =
      position.map((row, rowIndex) => {
        fieldColor = rowIndex % 2 === 0 ? "black" : "white";

        return row.map((el) => {
          fieldColor = fieldColor === "white" ? "black" : "white";

          return {
            field: fieldColor,
            figureType: el ?? "",
          };
        });
      });

    if (newPosition) {
      return newPosition;
    }

    return null;
  };

  const [currentPosition, setCurrentPosition] = useState<
    Array<Array<{ field: string; figureType: string }>>
  >([]);

  const [moves, setMoves] = useState<Array<string>>([]);

  useEffect(() => {
    if (!hasLoaded.current) {
      hasLoaded.current = true;

      api
        .post<InitTypes>("http://localhost:3000/init", "")
        .then((data) => {
          setFirstColorMove(data.int_config.color);
          setCurrentPosition(
            addFieldColorToPosition(data.int_config.prev_board) ?? []
          );

          setMoves((prevMoves) => [...prevMoves, data.stockfish.best_move]);
        })
        .catch((err) => {
          console.error(err);
        });
    }
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  useEffect(() => {
    const socket = new WebSocket("ws://127.0.0.1:3000/game");

    // Listen for messages
    socket.addEventListener("message", (event: { data: string }) => {
      const data: WebSocketData = JSON.parse(event.data) as WebSocketData;

      setCurrentPosition(
        addFieldColorToPosition(data.NextMove.raw_board) ?? []
      );
      setMoves((prevMoves) => [...prevMoves, data.NextMove.best_move]);
    });

    return () => socket.close();
    // eslint-disable-next-line react-hooks/exhaustive-deps
  }, []);

  return (
    <div style={{ display: "flex" }}>
      <div style={{ width: "80%" }}>
        {currentPosition.length && (
          <ChessBoard
            firstMove={firstColorMove}
            currentPosition={currentPosition}
          />
        )}
      </div>
      <div style={{ width: "20%" }}>
        <LastMovesSideBar
          firstColorMove={firstColorMove}
          moves={moves}
          currentPosition={currentPosition}
        />
      </div>
    </div>
  );
}

export default GameView;
