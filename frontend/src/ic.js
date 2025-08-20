import { createActor } from "./declarations/onchain360_backend";

export const backend = createActor(
  "uxrrr-q7777-77774-qaaaq-cai",
  {
    agentOptions: {
      host: "http://127.0.0.1:8000",
    },
  }
);