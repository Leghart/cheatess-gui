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

export interface SettingsType {
  verbose?: string;
  mode?: string;
  monitor?: { name?: string };
  stockfish?: {
    path?: string;
    elo?: number;
    skill?: number;
    depth?: number;
    hash?: number;
    pv?: number;
  };
  proc_image?: {
    margin?: number;
    piece_threshold?: number;
    extract_piece_threshold?: number;
    board_threshold?: number;
    difference_level?: number;
  };
  engine?: {
    pretty?: boolean;
  };
}
