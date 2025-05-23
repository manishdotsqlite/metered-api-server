import * as React from "react"

import { Button } from "@/components/ui/button"
import {
  Card,
  CardContent,
  CardDescription,
  CardFooter,
  CardHeader,
  CardTitle,
} from "@/components/ui/card"
import { Input } from "@/components/ui/input"
import { Label } from "@/components/ui/label"
import {
  Tooltip,
  TooltipContent,
  TooltipProvider,
  TooltipTrigger,
} from "@/components/ui/tooltip"
import { Info, InfoIcon } from "lucide-react"

export default function CardWithForm() {
  return (
    <Card className="w-[350px]">
      <CardHeader>
        <CardTitle>Enter username.</CardTitle>
        <CardDescription>Enter your username to use the addition API.</CardDescription>
      </CardHeader>
      <CardContent>
        <form>
          <div className="grid w-full items-center gap-4">
            <div className="flex flex-col space-y-1.5">
              <div className="flex items-center gap-2"><Label htmlFor="name">Username</Label>
              
              <TooltipProvider>
                <Tooltip>
                  <TooltipTrigger asChild>
                    <InfoIcon size={18}/>
                  </TooltipTrigger>
                  <TooltipContent>
                    <p>Use username "manish".</p>
                  </TooltipContent>
                </Tooltip>
              </TooltipProvider></div>
              
              <Input id="name" placeholder="manish" />
            </div>
          </div>
        </form>
      </CardContent>
      <CardFooter className="flex justify-end">
        <Button>Enter</Button>
      </CardFooter>
    </Card>
  )
}
