export interface InitTypes {
  int_config: {
    coords: Array<number>;
    prev_board: Array<Array<string>>;
    color: string;
  };
  ext_config: {
    verbose: string;
    mode: string;
    monitor: { number: number };
    stockfish: {
      path: string;
      elo: number;
      skill: number;
      depth: number;
      hash: number;
    };
    proc_image: {
      margin: string;
      piece_threshold: number;
      extract_piece_threshold: number;
      board_threshold: number;
      difference_level: number;
    };
    engine: { pretty: boolean };
  };
  stockfish: {
    version: string;
    best_move: string;
    eval: string;
  };
}

export interface WebSocketData {
  NextMove: {
    raw_board: Array<Array<string>>;
    eval: string;
    best_move: string;
  };
}
