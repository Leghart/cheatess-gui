import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";

interface Props {
  firstColorMove: string;
  moves: Array<string>;
  currentPosition: Array<Array<{ field: string; figureType: string }>>;
}

function LastMovesSideBar({ firstColorMove, moves, currentPosition }: Props) {
  const groupMoves = (moves: Array<string>): Array<Array<string>> => {
    const pairedMoves: Array<Array<string>> = [];
    let tmp: Array<string> = [];
    moves.forEach((move, index) => {
      tmp.push(move);
      if (tmp.length === 2) {
        pairedMoves.push(tmp);
        tmp = [];
      } else if (index === moves.length - 1) {
        pairedMoves.push([tmp[0], ""]);
      }
    });

    return pairedMoves;
  };

  const mapMoveToIndex = (move: string): Array<number> => {
    const letterMap = { a: 0, b: 1, c: 2, d: 3, e: 4, f: 5, g: 6, h: 7 };

    const letter = letterMap[move.charAt(0) as keyof typeof letterMap];
    const number =
      firstColorMove === "white"
        ? Math.abs(parseInt(move.charAt(1)) - 8)
        : parseInt(move.charAt(1));

    return [number, letter];
  };

  const showPieceToMove = (indexes: Array<number>): React.JSX.Element => {
    const [row, col] = indexes;

    if (currentPosition[row][col].figureType) {
      return (
        <img
          style={{ marginRight: "5px", width: "15px", height: "15px" }}
          src={`src/assets/pieces/${currentPosition[row][col].figureType}.png`}
        ></img>
      );
    }

    return <></>;
  };

  return (
    <div
      style={{
        border: "1px dashed black",
        color: "white",
        backgroundColor: "#4e4944",
      }}
    >
      <h2 style={{ marginBottom: "16px" }}> Table of top last moves</h2>
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead className="w-[20px] text-center">Move</TableHead>
            <TableHead className="text-center">{firstColorMove}</TableHead>
            <TableHead className="text-center">
              {firstColorMove === "white" ? "black" : "white"}
            </TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {groupMoves(moves).map((move, index) => (
            <TableRow key={index}>
              <TableCell className="font-medium">{index + 1}</TableCell>
              <TableCell className={!move[1] ? "best-move" : ""}>
                <div style={{ display: "flex", justifyContent: "center" }}>
                  {!move[1] && showPieceToMove(mapMoveToIndex(move[0]))}
                  {move[0]}
                </div>
              </TableCell>
              <TableCell
                className={
                  move[1] && groupMoves(moves).length - 1 === index
                    ? "best-move"
                    : ""
                }
              >
                <div style={{ display: "flex", justifyContent: "center" }}>
                  {move[1] &&
                    groupMoves(moves).length - 1 === index &&
                    showPieceToMove(mapMoveToIndex(move[1]))}
                  {move[1]}
                </div>
              </TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  );
}

export default LastMovesSideBar;
