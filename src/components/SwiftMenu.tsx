import { Button, Typography } from "@mui/joy";
import { Toolchain, useIDE } from "../utilities/IDEContext";
import ErrorIcon from "@mui/icons-material/Error";
import { SWIFT_VERSION_PREFIX } from "../utilities/constants";

export default () => {
  const { selectedToolchain, locateToolchain } = useIDE();

  return (
    <div
      style={{
        width: "fit-content",
        display: "flex",
        flexDirection: "column",
        gap: "var(--padding-md)",
      }}
    >
      {!selectedToolchain && (
        <Typography
          level="body-md"
          color="danger"
          sx={{
            alignContent: "center",
            display: "flex",
            gap: "var(--padding-xs)",
          }}
        >
          <ErrorIcon />
          No toolchain selected
        </Typography>
      )}
      {selectedToolchain !== null && !isCompatable(selectedToolchain) && (
        <Typography level="body-md" color="danger">
          Your selected toolchain is not compatible. Please select a swift{" "}
          {SWIFT_VERSION_PREFIX}
          toolchain.
        </Typography>
      )}
      {selectedToolchain !== null && isCompatable(selectedToolchain) && (
        <div>
          <Typography level="body-md">Selected toolchain:</Typography>
          <Typography level="body-sm">{selectedToolchain.path}</Typography>
          <Typography level="body-sm" color="success">
            Version: {selectedToolchain.version}
          </Typography>
        </div>
      )}
      <div
        style={{
          display: "flex",
          gap: "var(--padding-md)",
        }}
      >
        <Button variant="soft" onClick={locateToolchain}>
          Select Toolchain
        </Button>
      </div>
    </div>
  );
};

export function isCompatable(toolchain: Toolchain | null): boolean {
  if (!toolchain) return false;
  return toolchain.version.startsWith(SWIFT_VERSION_PREFIX);
}
