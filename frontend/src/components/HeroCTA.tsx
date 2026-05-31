"use client";

import { useWallet } from "@/lib/WalletProvider";
import { Button } from "@/components/ui/button";
import { Wallet } from "lucide-react";

export function HeroCTA() {
  const { isConnected, connect } = useWallet();

  const handleClick = () => {
    if (isConnected) {
      document.getElementById("explore-campaigns")?.scrollIntoView({ behavior: "smooth" });
    } else {
      connect();
    }
  };

  return (
    <Button
      size="lg"
      onClick={handleClick}
      className="mt-4 gap-2 transition-transform hover:scale-105"
    >
      <Wallet className="w-4 h-4" />
      {isConnected ? "Explore Campaigns" : "Get Started"}
    </Button>
  );
}
