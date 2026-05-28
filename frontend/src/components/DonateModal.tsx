"use client";

import { useState } from "react";
import { useDonate } from "@/hooks/useSoroban";
import { Campaign } from "@/lib/soroban";
import { Button } from "@/components/ui/button";
import {
  Dialog,
  DialogContent,
  DialogDescription,
  DialogFooter,
  DialogHeader,
  DialogTitle,
  DialogTrigger,
} from "@/components/ui/dialog";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import { Loader2 } from "lucide-react";

export function DonateModal({ campaign }: { campaign: Campaign }) {
  const [amount, setAmount] = useState("");
  const [isAnonymous, setIsAnonymous] = useState(false);
  const [isOpen, setIsOpen] = useState(false);
  const donate = useDonate();

  const handleDonate = async () => {
    if (donate.isPending) return; // Lock duplicate submissions
    if (!amount || isNaN(Number(amount))) {
      return;
    }

    try {
      await donate.mutateAsync({
        campaignId: campaign.id,
        amount,
        isAnonymous,
      });
      setIsOpen(false);
      setAmount("");
      setIsAnonymous(false);
    } catch (e: any) {
      // Errors are already handled by sonner inside useDonate mutation wrapper
      console.error(e);
    }
  };

  return (
    <Dialog open={isOpen} onOpenChange={(open) => {
      if (!donate.isPending) {
        setIsOpen(open);
      }
    }}>
      <DialogTrigger asChild>
        <Button className="flex-1">Donate Now</Button>
      </DialogTrigger>
      <DialogContent onPointerDownOutside={(e) => {
        if (donate.isPending) e.preventDefault(); // lock UI until resolution
      }} onEscapeKeyDown={(e) => {
        if (donate.isPending) e.preventDefault(); // lock UI until resolution
      }}>
        <DialogHeader>
          <DialogTitle>Donate to {campaign.title}</DialogTitle>
          <DialogDescription>
            Enter the amount of tokens you wish to contribute to this relief campaign.
          </DialogDescription>
        </DialogHeader>
        <div className="grid gap-4 py-4">
          <div className="grid gap-2">
            <Label htmlFor="amount">Amount</Label>
            <Input
              id="amount"
              type="number"
              placeholder="10.0"
              value={amount}
              onChange={(e) => setAmount(e.target.value)}
              disabled={donate.isPending}
            />
          </div>
          <div className="grid gap-1">
            <div className="flex items-center space-x-2 pt-2">
              <input
                id="anonymous"
                type="checkbox"
                checked={isAnonymous}
                onChange={(e) => setIsAnonymous(e.target.checked)}
                disabled={donate.isPending}
                className="h-4 w-4 rounded border-gray-300 text-primary focus:ring-primary accent-primary cursor-pointer"
              />
              <Label htmlFor="anonymous" className="cursor-pointer select-none text-sm font-medium leading-none peer-disabled:cursor-not-allowed peer-disabled:opacity-70">
                Donate anonymously
              </Label>
            </div>
            <p className="text-xs text-muted-foreground mt-1 leading-relaxed">
              Hides your address in the public event feed and leaderboard. Ledger records will still show the transfer.
            </p>
          </div>
        </div>
        <DialogFooter>
          <Button variant="outline" onClick={() => setIsOpen(false)} disabled={donate.isPending}>
            Cancel
          </Button>
          <Button onClick={handleDonate} disabled={donate.isPending}>
            {donate.isPending ? (
              <>
                <Loader2 className="mr-2 h-4 w-4 animate-spin" />
                Donating...
              </>
            ) : (
              "Confirm Donation"
            )}
          </Button>
        </DialogFooter>
      </DialogContent>
    </Dialog>
  );
}
