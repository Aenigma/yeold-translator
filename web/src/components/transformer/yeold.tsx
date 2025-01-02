import React, { useCallback, useReducer } from "react";
import { Wand2 } from "lucide-react";
import { useDebounceCallback } from "usehooks-ts";

import { AutoRP } from "yeold";

import { Card, CardContent, CardHeader, CardTitle } from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Button } from "@/components/ui/button";
import { Checkbox } from "@/components/ui/checkbox";
import { Label } from "@/components/ui/label";

const arp = AutoRP.default();

interface ArpState {
  input: string;
  output: string;
  prepend: boolean;
  append: boolean;
}

const DEFAULT_ARP_STATE: ArpState = Object.freeze({
  input: "",
  output: "",
  prepend: true,
  append: true,
});

type ArpAction =
  | { type: "INPUT_TRANSLATE"; payload: string }
  | { type: "UPDATE"; payload: Partial<ArpState> }
  | { type: "TRANSLATE" };

function arpReducer(
  state: ArpState = DEFAULT_ARP_STATE,
  action: ArpAction
): ArpState {
  switch (action.type) {
    case "INPUT_TRANSLATE": {
      if (action.payload === state.input) {
        return state;
      }

      return {
        ...state,
        input: action.payload,
        output: arp.translate_postprocess(
          action.payload,
          state.prepend,
          state.append
        ),
      };
    }
    case "UPDATE": {
      return {
        ...state,
        ...action.payload,
      };
    }
    case "TRANSLATE": {
      const output = state.input
        ? arp.translate_postprocess(state.input, state.prepend, state.append)
        : "";
      if (output == state.output) {
        return state;
      }
      return {
        ...state,
        output,
      };
    }
    default:
      return state;
  }
}

const YeoldTransformer = () => {
  const [state, dispatch] = useReducer(arpReducer, DEFAULT_ARP_STATE);

  const translateDispatch = useDebounceCallback(
    useCallback(() => dispatch({ type: "TRANSLATE" }), []),
    200
  );

  const onChange = (e: React.ChangeEvent<HTMLInputElement>) => {
    dispatch({ type: "UPDATE", payload: { input: e.target.value } });
    translateDispatch();
  };

  const onPrependChange = (checked: boolean) => {
    dispatch({ type: "UPDATE", payload: { prepend: checked } });
    translateDispatch();
  };

  const onAppendChange = (checked: boolean) => {
    dispatch({ type: "UPDATE", payload: { append: checked } });
    translateDispatch();
  };

  const onSubmit = (e: React.FormEvent<HTMLFormElement>) => {
    e.preventDefault();
    if (translateDispatch.isPending()) {
      translateDispatch.cancel();
    }
    dispatch({ type: "TRANSLATE" });
  };

  return (
    <form onSubmit={onSubmit}>
      <Card className="w-full max-w-2xl mx-auto">
        <CardHeader>
          <CardTitle className="text-2xl font-bold">
            Yeold Text Transformer
          </CardTitle>
        </CardHeader>
        <CardContent className="space-y-6">
          <div className="space-y-2">
            <Label htmlFor="transform">Input Text</Label>
            <div className="flex gap-2">
              <Input
                id="transform"
                name="transform"
                autoComplete="off"
                placeholder="Enter text to transform..."
                value={state.input}
                onChange={onChange}
                className="flex-1"
              />
              <Button className="bg-gradient-to-r from-purple-500 to-blue-500 hover:from-purple-600 hover:to-blue-600">
                <Wand2 className="w-4 h-4 mr-2" />
                Transform
              </Button>
            </div>
            <div className="flex items-center space-x-4">
              <div className="flex items-center space-x-2">
                <Checkbox
                  id="prepend"
                  name="prepend"
                  checked={state.prepend}
                  onCheckedChange={onPrependChange}
                />
                <Label htmlFor="prepend">Prepend</Label>
              </div>

              <div className="flex items-center space-x-2">
                <Checkbox
                  id="append"
                  name="append"
                  checked={state.append}
                  onCheckedChange={onAppendChange}
                />
                <Label htmlFor="append">Append</Label>
              </div>
            </div>
          </div>

          {state.output && (
            <div className="space-y-2">
              <label className="text-sm font-medium">Transformed Text</label>
              <div className="p-4 rounded-lg bg-slate-50 dark:bg-slate-900">
                <p className="font-mono">{state.output}</p>
              </div>
            </div>
          )}
        </CardContent>
      </Card>
    </form>
  );
};

export default YeoldTransformer;
