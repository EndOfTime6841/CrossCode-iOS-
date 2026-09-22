import React from "react";

interface Props {
  children: React.ReactNode;
  label: string;
}

interface State {
  error: Error | null;
}

export default class ErrorBoundary extends React.Component<Props, State> {
  constructor(props: Props) {
    super(props);
    this.state = { error: null };
  }

  static getDerivedStateFromError(error: Error): State {
    return { error };
  }

  render() {
    if (this.state.error) {
      return (
        <div
          style={{
            padding: "1rem",
            border: "2px solid red",
            borderRadius: "8px",
            color: "red",
            fontFamily: "monospace",
            whiteSpace: "pre-wrap",
          }}
        >
          <strong>{this.props.label} crashed:</strong>
          {"\n"}
          {this.state.error.message}
          {"\n\n"}
          {this.state.error.stack}
        </div>
      );
    }
    return this.props.children;
  }
}
