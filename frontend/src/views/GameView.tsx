interface Props {
  exampleProp: string;
}

function GameView({ exampleProp }: Props) {
  return <div>GameView {exampleProp}</div>;
}

export default GameView;
