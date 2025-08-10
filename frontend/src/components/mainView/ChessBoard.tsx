// Create array to map chessboard, then render html based on this array
// Array will looks like [[{field: black/white, figureType: chessPiece},...], [{field: black/white, figureType: chessPiece},...}]]
// Html will looks like:
/**
 * <tbody>
 * for i in range(8):
 * <tr>
 *  for j in range(8):
 *    <td>
 *      <div background-color: array[i][j].field><img src="assets/chessPieces/array[i][j].figureType" /></div>
 *    </td>
 *  </tr>
 * </tbody>
 */
import { useState } from "react";
import { Table, TableBody, TableCell, TableRow } from "@/components/ui/table";

function ChessBoard() {
  const initialPosition = [
    [
      { field: "white", figureType: "br" },
      { field: "black", figureType: "bn" },
      { field: "white", figureType: "bb" },
      { field: "black", figureType: "bq" },
      { field: "white", figureType: "bk" },
      { field: "black", figureType: "bb" },
      { field: "white", figureType: "bn" },
      { field: "black", figureType: "br" },
    ],
    [
      { field: "black", figureType: "bp" },
      { field: "white", figureType: "bp" },
      { field: "black", figureType: "bp" },
      { field: "white", figureType: "bp" },
      { field: "black", figureType: "bp" },
      { field: "white", figureType: "bp" },
      { field: "black", figureType: "bp" },
      { field: "white", figureType: "bp" },
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
      { field: "white", figureType: "wp" },
      { field: "black", figureType: "wp" },
      { field: "white", figureType: "wp" },
      { field: "black", figureType: "wp" },
      { field: "white", figureType: "wp" },
      { field: "black", figureType: "wp" },
      { field: "white", figureType: "wp" },
      { field: "black", figureType: "wp" },
    ],
    [
      { field: "black", figureType: "wr" },
      { field: "white", figureType: "wn" },
      { field: "black", figureType: "wb" },
      { field: "white", figureType: "wq" },
      { field: "black", figureType: "wk" },
      { field: "white", figureType: "wb" },
      { field: "black", figureType: "wn" },
      { field: "white", figureType: "wr" },
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

  return (
    <div className="chessBoardWrapper">
      <Table className="chessBoard" style={{ width: "800px" }}>
        {/* <TableHeader>
          <TableRow>
            <TableHead className="text-center">A</TableHead>
            <TableHead className="text-center">B</TableHead>
            <TableHead className="text-center">C</TableHead>
            <TableHead className="text-center">D</TableHead>
            <TableHead className="text-center">E</TableHead>
            <TableHead className="text-center">F</TableHead>
            <TableHead className="text-center">G</TableHead>
            <TableHead className="text-center">H</TableHead>
          </TableRow>
        </TableHeader> */}
        <TableBody>{createBody()}</TableBody>
      </Table>
    </div>
  );
}

export default ChessBoard;
