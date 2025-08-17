import { useEffect, useState } from "react";
import { Table, TableBody, TableCell, TableRow } from "@/components/ui/table";

interface Props {
  firstMove: string;
}

function ChessBoard({ firstMove }: Props) {
  const initialPosition = [
    [
      { field: "black", figureType: "br" },
      { field: "white", figureType: "bn" },
      { field: "black", figureType: "bb" },
      { field: "white", figureType: "bq" },
      { field: "black", figureType: "bk" },
      { field: "white", figureType: "bb" },
      { field: "black", figureType: "bn" },
      { field: "white", figureType: "br" },
    ],
    [
      { field: "white", figureType: "bp" },
      { field: "black", figureType: "bp" },
      { field: "white", figureType: "bp" },
      { field: "black", figureType: "bp" },
      { field: "white", figureType: "bp" },
      { field: "black", figureType: "bp" },
      { field: "white", figureType: "bp" },
      { field: "black", figureType: "bp" },
    ],
    [
      { field: "black", figureType: "" },
      { field: "white", figureType: "" },
      { field: "black", figureType: "" },
      { field: "white", figureType: "" },
      { field: "black", figureType: "" },
      { field: "white", figureType: "" },
      { field: "black", figureType: "" },
      { field: "white", figureType: "" },
    ],
    [
      { field: "white", figureType: "" },
      { field: "black", figureType: "" },
      { field: "white", figureType: "" },
      { field: "black", figureType: "" },
      { field: "white", figureType: "" },
      { field: "black", figureType: "" },
      { field: "white", figureType: "" },
      { field: "black", figureType: "" },
    ],
    [
      { field: "black", figureType: "" },
      { field: "white", figureType: "" },
      { field: "black", figureType: "" },
      { field: "white", figureType: "" },
      { field: "black", figureType: "" },
      { field: "white", figureType: "" },
      { field: "black", figureType: "" },
      { field: "white", figureType: "" },
    ],
    [
      { field: "white", figureType: "" },
      { field: "black", figureType: "" },
      { field: "white", figureType: "" },
      { field: "black", figureType: "" },
      { field: "white", figureType: "" },
      { field: "black", figureType: "" },
      { field: "white", figureType: "" },
      { field: "black", figureType: "" },
    ],
    [
      { field: "black", figureType: "wp" },
      { field: "white", figureType: "wp" },
      { field: "black", figureType: "wp" },
      { field: "white", figureType: "wp" },
      { field: "black", figureType: "wp" },
      { field: "white", figureType: "wp" },
      { field: "black", figureType: "wp" },
      { field: "white", figureType: "wp" },
    ],
    [
      { field: "white", figureType: "wr" },
      { field: "black", figureType: "wn" },
      { field: "white", figureType: "wb" },
      { field: "black", figureType: "wq" },
      { field: "white", figureType: "wk" },
      { field: "black", figureType: "wb" },
      { field: "white", figureType: "wn" },
      { field: "black", figureType: "wr" },
    ],
  ];

  const [currentPosition, setCurrentPosition] = useState(initialPosition);

  const createRow = (row: number) => {
    const listItems = [];

    for (let col = 0; col < 8; col++) {
      listItems.push(
        <TableCell
          key={col}
          style={{
            backgroundColor:
              currentPosition[row][col].field === "white"
                ? "#eeeed2"
                : "#769656",
          }}
        >
          {currentPosition[row][col].figureType && (
            <img
              style={{ margin: "auto" }}
              src={`src/assets/pieces/${currentPosition[row][col].figureType}.png`}
            ></img>
          )}
        </TableCell>
      );
    }

    return listItems;
  };

  const createBody = () => {
    const listItems = [];

    for (let col = 0; col < 8; col++) {
      listItems.push(<TableRow key={col}>{createRow(col)}</TableRow>);
    }

    return listItems;
  };

  const createCoordinates = () => {
    const startPosition = {
      letters: { x: 85, y: 795 },
      numbers: { x: 2, y: 720 },
    };

    if (firstMove !== "white") {
      startPosition.letters = { x: 785, y: 795 };
      startPosition.numbers = { x: 2, y: 20 };
    }

    const numbers: Array<React.JSX.Element> = [];
    const letters: Array<React.JSX.Element> = [];

    [
      [1, "a"],
      [2, "b"],
      [3, "c"],
      [4, "d"],
      [5, "e"],
      [6, "f"],
      [7, "g"],
      [8, "h"],
    ].forEach(([num, letter], index) => {
      const calcNumY =
        firstMove === "white"
          ? startPosition.numbers.y - 100 * index
          : startPosition.numbers.y + 100 * index;
      const calcLetX =
        firstMove === "white"
          ? startPosition.letters.x + 100 * index
          : startPosition.letters.x - 100 * index;

      numbers.push(
        <text x={startPosition.numbers.x} y={calcNumY} fontSize="20">
          {num}
        </text>
      );
      letters.push(
        <text x={calcLetX} y={startPosition.letters.y} fontSize="20">
          {letter}
        </text>
      );
    });

    return (
      <svg className="coordinates" aria-hidden="true">
        {...numbers}
        {...letters}
      </svg>
    );
  };

  return (
    <div className="chessBoardWrapper">
      <Table className="chessBoard" style={{ width: "800px" }}>
        {createCoordinates()}
        <TableBody>{createBody()}</TableBody>
      </Table>
    </div>
  );
}

export default ChessBoard;
