interface Props {
  exampleProp: string;
}

function Settings({ exampleProp }: Props) {
  return <div>Settings {exampleProp}</div>;
}

export default Settings;
