export interface InitTypes {
  int_config: {
    coords: Array<number>;
    prev_board: Array<Array<string>>;
    color: string;
  };
  stockfish: {
    version: string;
    summary: Array<SummaryOfMove>;
  };
}

export interface WebSocketData {
  NextMove: {
    last_move: string;
    summary: Array<SummaryOfMove>;
    raw_board: Array<Array<string>>;
  };
}

interface SummaryOfMove {
  evaluation: string;
  main_line: Array<string>;
}
