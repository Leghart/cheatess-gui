import { Button } from "@/components/ui/button";
import {
  Collapsible,
  CollapsibleContent,
  CollapsibleTrigger,
} from "@/components/ui/collapsible";
import {
  DialogClose,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogContent,
  Dialog,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { ChevronsUpDown } from "lucide-react";
import { useState, type FormEvent } from "react";
import {
  Select,
  SelectContent,
  SelectGroup,
  SelectItem,
  SelectTrigger,
  SelectValue,
} from "@/components/ui/select";
import type { SettingsType } from "@/types/RequestTypes";

function Settings() {
  const [isStockfishSettingsOpen, setStockfishOpen] = useState(false);
  const [isProcSettingsOpen, setProcOpen] = useState(false);
  const [isEngineSettingsOpen, setEngineOpen] = useState(false);

  const [formData, setFormData] = useState<SettingsType>({});

  function test(e: FormEvent): void {
    // const requestData: Partial<SettingsType> = {};
    e.preventDefault();

    // const form = e.target;
    // const formData = new FormData(form);

    // formData.forEach((value, key) => {
    //   if (key === "pretty") {
    //     requestData[key] = value === "true";
    //   } else {
    //     requestData[key] = value;
    //   }
    // });

    console.log(formData);
  }

  return (
    <Dialog>
      <DialogTrigger asChild>
        <Button className="mt-2 mb-2" variant="outline">
          Settings
        </Button>
      </DialogTrigger>
      <DialogContent
        className="sm:max-w-[425px]"
        onInteractOutside={(e) => e.preventDefault()}
        style={{ maxHeight: "80vh", overflowY: "scroll" }}
      >
        <form onSubmit={test}>
          <DialogHeader className="mb-4">
            <DialogTitle>Cheatess settings</DialogTitle>
            <DialogDescription>
              Make sure that Cheatess will work as you expect
            </DialogDescription>
          </DialogHeader>
          <div className="grid gap-4">
            <div className="grid gap-3">
              <Label htmlFor="verbose">Verbose</Label>
              <Input
                name="verbose"
                value={formData.verbose}
                onChange={(e) =>
                  setFormData({ ...formData, verbose: e.target.value })
                }
              />
            </div>

            <div className="grid gap-3">
              <Label htmlFor="mode">Mode</Label>
              <Input
                name="mode"
                value={formData.mode}
                onChange={(e) =>
                  setFormData({ ...formData, mode: e.target.value })
                }
              />
            </div>

            <div className="grid gap-3">
              <Label htmlFor="monitor">Monitor name</Label>
              <Input
                name="monitor"
                value={formData.monitor?.name}
                onChange={(e) =>
                  setFormData({
                    ...formData,
                    monitor: { name: e.target.value },
                  })
                }
              />
            </div>

            <Collapsible
              open={isStockfishSettingsOpen}
              onOpenChange={setStockfishOpen}
              className="flex w-[350px] flex-col gap-2"
            >
              <div className="flex items-center justify-between gap-4 px-4">
                <h4 className="text-sm font-semibold mb-2">
                  Stockfish options
                </h4>
                <CollapsibleTrigger asChild>
                  <Button variant="ghost" size="icon" className="size-8">
                    <ChevronsUpDown />
                    <span className="sr-only">Toggle</span>
                  </Button>
                </CollapsibleTrigger>
              </div>
              <CollapsibleContent className="flex flex-col gap-2">
                <div className="grid gap-3">
                  <Label htmlFor="Path">Path</Label>
                  <Input
                    name="Path"
                    value={formData.stockfish?.path}
                    onChange={(e) =>
                      setFormData({
                        ...formData,
                        stockfish: {
                          ...formData.stockfish,
                          path: e.target.value,
                        },
                      })
                    }
                  />
                </div>

                <div className="grid gap-3">
                  <Label htmlFor="Path">Elo</Label>
                  <Input
                    name="Elo"
                    value={formData.stockfish?.elo}
                    onChange={(e) =>
                      setFormData({
                        ...formData,
                        stockfish: {
                          ...formData.stockfish,
                          elo: Number(e.target.value),
                        },
                      })
                    }
                  />
                </div>

                <div className="grid gap-3">
                  <Label htmlFor="skill">Skill</Label>
                  <Input
                    name="skill"
                    value={formData.stockfish?.skill}
                    onChange={(e) =>
                      setFormData({
                        ...formData,
                        stockfish: {
                          ...formData.stockfish,
                          skill: Number(e.target.value),
                        },
                      })
                    }
                  />
                </div>

                <div className="grid gap-3">
                  <Label htmlFor="depth">Depth</Label>
                  <Input
                    name="depth"
                    value={formData.stockfish?.depth}
                    onChange={(e) =>
                      setFormData({
                        ...formData,
                        stockfish: {
                          ...formData.stockfish,
                          depth: Number(e.target.value),
                        },
                      })
                    }
                  />
                </div>

                <div className="grid gap-3">
                  <Label htmlFor="hash">Hash</Label>
                  <Input
                    name="hash"
                    value={formData.stockfish?.hash}
                    onChange={(e) =>
                      setFormData({
                        ...formData,
                        stockfish: {
                          ...formData.stockfish,
                          hash: Number(e.target.value),
                        },
                      })
                    }
                  />
                </div>

                <div className="grid gap-3">
                  <Label htmlFor="pv">Pv</Label>
                  <Input
                    name="pv"
                    value={formData.stockfish?.pv}
                    onChange={(e) =>
                      setFormData({
                        ...formData,
                        stockfish: {
                          ...formData.stockfish,
                          pv: Number(e.target.value),
                        },
                      })
                    }
                  />
                </div>
              </CollapsibleContent>
            </Collapsible>

            <Collapsible
              open={isProcSettingsOpen}
              onOpenChange={setProcOpen}
              className="flex w-[350px] flex-col gap-2"
            >
              <div className="flex items-center justify-between gap-4 px-4">
                <h4 className="text-sm font-semibold mb-2">Image options</h4>
                <CollapsibleTrigger asChild>
                  <Button variant="ghost" size="icon" className="size-8">
                    <ChevronsUpDown />
                    <span className="sr-only">Toggle</span>
                  </Button>
                </CollapsibleTrigger>
              </div>
              <CollapsibleContent className="flex flex-col gap-2">
                <div className="grid gap-3">
                  <Label htmlFor="margin">Margin</Label>
                  <Input
                    name="margin"
                    type="number"
                    value={formData.proc_image?.margin}
                    onChange={(e) =>
                      setFormData({
                        ...formData,
                        // eslint-disable-next-line camelcase
                        proc_image: {
                          ...formData.proc_image,
                          margin: Number(e.target.value),
                        },
                      })
                    }
                  />
                </div>

                <div className="grid gap-3">
                  <Label htmlFor="piece_threshold">Piece threshold</Label>
                  <Input
                    name="piece_threshold"
                    type="number"
                    step={0.01}
                    value={formData.proc_image?.piece_threshold}
                    onChange={(e) =>
                      setFormData({
                        ...formData,
                        // eslint-disable-next-line camelcase
                        proc_image: {
                          ...formData.proc_image,
                          // eslint-disable-next-line camelcase
                          piece_threshold: Number(e.target.value),
                        },
                      })
                    }
                  />
                </div>

                <div className="grid gap-3">
                  <Label htmlFor="extract_piece_threshold">
                    Extract piece threshold
                  </Label>
                  <Input
                    name="extract_piece_threshold"
                    type="number"
                    step={0.01}
                    value={formData.proc_image?.extract_piece_threshold}
                    onChange={(e) =>
                      setFormData({
                        ...formData,
                        // eslint-disable-next-line camelcase
                        proc_image: {
                          ...formData.proc_image,
                          // eslint-disable-next-line camelcase
                          extract_piece_threshold: Number(e.target.value),
                        },
                      })
                    }
                  />
                </div>

                <div className="grid gap-3">
                  <Label htmlFor="board_threshold">Board threshold</Label>
                  <Input
                    name="board_threshold"
                    type="number"
                    step={0.01}
                    value={formData.proc_image?.board_threshold}
                    onChange={(e) =>
                      setFormData({
                        ...formData,
                        // eslint-disable-next-line camelcase
                        proc_image: {
                          ...formData.proc_image,
                          // eslint-disable-next-line camelcase
                          board_threshold: Number(e.target.value),
                        },
                      })
                    }
                  />
                </div>

                <div className="grid gap-3">
                  <Label htmlFor="difference_level">Difference level</Label>
                  <Input
                    name="difference_level"
                    type="number"
                    step={1}
                    value={formData.proc_image?.difference_level}
                    onChange={(e) =>
                      setFormData({
                        ...formData,
                        // eslint-disable-next-line camelcase
                        proc_image: {
                          ...formData.proc_image,
                          // eslint-disable-next-line camelcase
                          difference_level: Number(e.target.value),
                        },
                      })
                    }
                  />
                </div>
              </CollapsibleContent>
            </Collapsible>

            <Collapsible
              open={isEngineSettingsOpen}
              onOpenChange={setEngineOpen}
              className="flex w-[350px] flex-col gap-2"
            >
              <div className="flex items-center justify-between gap-4 px-4">
                <h4 className="text-sm font-semibold">Engine options</h4>
                <CollapsibleTrigger asChild>
                  <Button variant="ghost" size="icon" className="size-8">
                    <ChevronsUpDown />
                    <span className="sr-only">Toggle</span>
                  </Button>
                </CollapsibleTrigger>
              </div>
              <CollapsibleContent className="flex flex-col gap-2">
                <div className="grid gap-3">
                  <Label htmlFor="pretty">Pretty</Label>
                  <Select
                    name="pretty"
                    value={JSON.stringify(formData.engine?.pretty) ?? "false"}
                    onValueChange={(value) =>
                      setFormData({
                        ...formData,
                        engine: {
                          ...formData.engine,
                          pretty: JSON.parse(value) as boolean,
                        },
                      })
                    }
                  >
                    <SelectTrigger className="w-full max-w-48">
                      <SelectValue />
                    </SelectTrigger>
                    <SelectContent>
                      <SelectGroup>
                        <SelectItem value={"true"}>Yes</SelectItem>
                        <SelectItem value={"false"}>No</SelectItem>
                      </SelectGroup>
                    </SelectContent>
                  </Select>
                </div>
              </CollapsibleContent>
            </Collapsible>
          </div>
          <DialogFooter className="mt-6">
            <DialogClose asChild>
              <Button variant="outline">Cancel</Button>
            </DialogClose>
            <Button type="submit">Save changes</Button>
          </DialogFooter>
        </form>
      </DialogContent>
    </Dialog>
  );
}

export default Settings;
