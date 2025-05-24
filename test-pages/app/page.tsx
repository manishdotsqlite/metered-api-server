"use client";

import * as React from "react";
import { Button } from "@/components/ui/button";
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card";
import { Input } from "@/components/ui/input";
import { Label } from "@/components/ui/label";
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip";
import { InfoIcon } from "lucide-react";
import axios from "axios";
import { toast, Toaster } from "sonner";
import { setSecureCookie } from "@/lib/cookie-utils";
import { useRouter } from "next/navigation";

export default function Login() {
  const [username, setUsername] = React.useState<string>("");
  const router = useRouter();

  const onSubmit = async () => {
    const validation = await axios.get(
      `http://127.0.0.1:3031/validate/${username}`
    );
    const ssun = await validation.data.data;
    if (ssun === username) {
      setSecureCookie("username", ssun, 7);
      toast.success("You have been logged in.");
      router.push("/test");
      return;
    } else {
      toast.error("Credentials are not correct.");
      return;
    }
  };

  return (
    <>
      <Card className="w-[350px]">
        <CardHeader>
          <CardTitle>Enter username.</CardTitle>
          <CardDescription>
            Enter your username to use the addition API.
          </CardDescription>
        </CardHeader>
        <CardContent>
          <form>
            <div className="grid w-full items-center gap-4">
              <div className="flex flex-col space-y-1.5">
                <div className="flex items-center gap-2">
                  <Label htmlFor="name">Username</Label>

                  <TooltipProvider>
                    <Tooltip>
                      <TooltipTrigger asChild>
                        <InfoIcon size={18} />
                      </TooltipTrigger>
                      <TooltipContent>
                        <p>Use username &quot;manish&quot;.</p>
                      </TooltipContent>
                    </Tooltip>
                  </TooltipProvider>
                </div>

                <Input
                  id="name"
                  placeholder="manish"
                  value={username}
                  onChange={(e) => setUsername(e.target.value)}
                />
              </div>
            </div>
          </form>
        </CardContent>
        <CardFooter className="flex justify-end">
          <Button onClick={onSubmit}>Enter</Button>
        </CardFooter>
      </Card>
      <Toaster />
    </>
  );
}
