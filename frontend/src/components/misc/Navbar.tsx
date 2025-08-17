import { Button } from "@/components/ui/button";

function Navbar() {
  return (
    <div
      className="navbar"
      style={{ display: "flex", justifyContent: "flex-end" }}
    >
      <Button>Settings</Button>
    </div>
  );
}

export default Navbar;
