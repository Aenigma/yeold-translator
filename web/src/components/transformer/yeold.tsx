import React, { useCallback, useReducer } from "react";

import { AutoRP } from "yeold";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";

const arp = AutoRP.default();

interface ArpState {
  input: string;
  output: string;
}

function arpReducer(
  state: ArpState = { input: "", output: "" },
  action: string
) {
  if (action === state.input) {
    return state;
  }

  return {
    input: action,
    output: arp.translate_postprocess(action, true, true),
  };
}

const YeoldTransformer = () => {
  const [{ input, output }, dispatch] = useReducer(arpReducer, {
    input: "",
    output: "",
  });

  const onChange = useCallback((e: React.ChangeEvent<HTMLInputElement>) => {
    dispatch(e.target.value);
  }, []);

  return (
    <Card className="w-full max-w-2xl mx-auto">
      <CardHeader>
        <CardTitle className="text-2xl font-bold">
          Yeold Text Transformer
        </CardTitle>
      </CardHeader>
      <CardContent className="space-y-6">
        <div className="space-y-2">
          <label className="text-sm font-medium">Input Text</label>
          <div className="flex gap-2">
            <Input
              placeholder="Enter text to transform..."
              value={input}
              onChange={onChange}
              className="flex-1"
            />
          </div>
        </div>

        <div className="space-y-2">
          <label className="text-sm font-medium">Transformed Text</label>
          <div className="p-4 rounded-lg bg-slate-50 dark:bg-slate-900">
            <p className="font-mono">{output || "\u00A0"}</p>
          </div>
        </div>
      </CardContent>
    </Card>
  );
};

export default YeoldTransformer;
