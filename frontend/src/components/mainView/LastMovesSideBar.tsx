import {
  Table,
  TableBody,
  TableCell,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";

function LastMovesSideBar({
  firstColorMove,
  moves,
}: {
  firstColorMove: string;
  moves: Array<string>;
}) {
  return (
    <div style={{ border: "1px dashed black", color: "white" }}>
      <h2 style={{ marginBottom: "16px" }}> Table of last moves</h2>
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
          {moves.map((_, index) => {
            if (index % 2 === 0) {
              return (
                <TableRow key={index}>
                  <TableCell className="font-medium">{index + 1}</TableCell>
                  <TableCell>{moves[index]}</TableCell>
                  <TableCell>{moves[index + 1]}</TableCell>
                </TableRow>
              );
            }
            return;
          })}
        </TableBody>
      </Table>
    </div>
  );
}

export default LastMovesSideBar;
