import {
    Card,
    CardContent,
    CardDescription,
    CardFooter,
    CardHeader,
    CardTitle,
  } from "@/components/ui/card"
import { Avatar, AvatarFallback, AvatarImage } from "../ui/avatar"

  export function Message() {
    return (
        <Card>
        <CardHeader>
          <Avatar>
            <AvatarImage src="https://avatars.githubusercontent.com/u/82332291?v=4" alt="@Hukasx0" />
            <AvatarFallback>H</AvatarFallback>
          </Avatar>
          <CardTitle>Assistant</CardTitle>
          <CardDescription>at 17:07 22.03.2024</CardDescription>
        </CardHeader>
        <CardContent>
          <p>Hello user!</p>
        </CardContent>
        <CardFooter>
          {/*<p>Card Footer</p>*/}
        </CardFooter>
      </Card>
    )
  }