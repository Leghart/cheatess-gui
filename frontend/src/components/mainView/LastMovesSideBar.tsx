import { useState } from "react";
import {
  Table,
  TableBody,
  TableCaption,
  TableCell,
  TableFooter,
  TableHead,
  TableHeader,
  TableRow,
} from "@/components/ui/table";

function LastMovesSideBar() {
  const defaultLastMoves = [
    ["e4", "e5"],
    ["Nf3", "Nc6"],
    ["Bc4", "Bc5"],
    ["b4", "Bxb4"],
  ];

  const [lastMoves, setLastMoves] = useState(defaultLastMoves);

  return (
    <div style={{ border: "1px dashed black", color: "white" }}>
      <h2 style={{ marginBottom: "16px" }}> Table of last moves</h2>
      <Table>
        <TableHeader>
          <TableRow>
            <TableHead className="w-[20px] text-center">Move</TableHead>
            <TableHead className="text-center">White</TableHead>
            <TableHead className="text-center">Black</TableHead>
          </TableRow>
        </TableHeader>
        <TableBody>
          {lastMoves.map((move, index) => (
            <TableRow key={index}>
              <TableCell className="font-medium">{index + 1}</TableCell>
              <TableCell>{move[0]}</TableCell>
              <TableCell>{move[1]}</TableCell>
            </TableRow>
          ))}
        </TableBody>
      </Table>
    </div>
  );
}

export default LastMovesSideBar;
