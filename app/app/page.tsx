import { Card, CardHeader, CardTitle, CardContent } from "@/components/ui/card"
import { Separator } from "@/components/ui/separator"
import { Button } from "@/components/ui/button"
import { Avatar, AvatarImage, AvatarFallback } from "@/components/ui/avatar"
import { Textarea } from "@/components/ui/textarea"
import { Plus } from "lucide-react"

export default function Home() {
  return (
    <div className="flex h-[100dvh] w-full">

      {/* Main Content */}
      <main className="flex-grow relative px-6 py-6 overflow-y-auto w-full">
        <div className="space-y-6">
          <Card>
            <CardHeader>
              <CardTitle className="text-lg">Post something</CardTitle>
            </CardHeader>
            <CardContent>
              <div className="flex gap-4 items-start">
                <Avatar>
                  <AvatarImage src="/avatars/user.png" />
                  <AvatarFallback>U</AvatarFallback>
                </Avatar>
                <div className="flex-1 space-y-2">
                  <Textarea placeholder="What's happening?" className="resize-none" />
                  <div className="flex justify-end">
                    <Button size="sm">Tweet</Button>
                  </div>
                </div>
              </div>
            </CardContent>
          </Card>

          <Separator />

          <Card>
            <CardHeader>
              <CardTitle className="text-xl">Home Feed</CardTitle>
            </CardHeader>
            <CardContent>
              <p className="text-muted-foreground">
                No tweets yet. Be the first to post.
              </p>
            </CardContent>
          </Card>
        </div>

        <Button
          className="absolute bottom-6 right-6 rounded-full h-12 w-12 p-0 shadow-lg"
          variant="default"
        >
          <Plus className="w-5 h-5" />
        </Button>
      </main>
    </div>
  )
}
