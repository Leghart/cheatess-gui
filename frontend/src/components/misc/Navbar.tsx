import Settings from "@/views/Settings";

function Navbar() {
  return (
    <div
      className="navbar"
      style={{
        display: "flex",
        justifyContent: "flex-end",
        backgroundColor: "#b6b2a1",
        width: "100%",
        padding: "0 16px",
      }}
    >
      <Settings />
    </div>
  );
}

export default Navbar;
