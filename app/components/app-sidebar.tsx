import { User2, Home, Inbox, Search, Settings } from "lucide-react"
import {
  Sidebar,
  SidebarContent,
  SidebarFooter,
  SidebarGroup,
  SidebarGroupContent,
  SidebarGroupLabel,
  SidebarMenu,
  SidebarMenuButton,
  SidebarMenuItem
} from "@/components/ui/sidebar"
import { Button } from "@/components/ui/button"
import { useWallet } from "@solana/wallet-adapter-react";
import { WalletButton } from "@/components/providers/wallet-provider";

const items = [
    {
      title: "Feed",
      url: "/",
      icon: Home,
    },
    {
      title: "Explore",
      url: "/explore",
      icon: Search,
    },
    {
      title: "Notifications",
      url: "/notifications",
      icon: Inbox,
    },
    {
      title: "Profile",
      url: "/profile",
      icon: User2, 
    },
    {
      title: "Settings",
      url: "/settings",
      icon: Settings,
    },
  ]
  

export function AppSidebar() {
  const { connected } = useWallet();
  return (
    <Sidebar>
      <SidebarContent>
        <SidebarGroup>
          <SidebarGroupLabel>DTwitter</SidebarGroupLabel>
          <SidebarGroupContent>
            <SidebarMenu>
              {items.map((item) => (
                <SidebarMenuItem key={item.title}>
                  <SidebarMenuButton asChild>
                    <a href={item.url} className="flex items-center gap-2">
                      <item.icon className="h-4 w-4" />
                      <span>{item.title}</span>
                    </a>
                  </SidebarMenuButton>
                </SidebarMenuItem>
              ))}
            </SidebarMenu>
          </SidebarGroupContent>
        </SidebarGroup>
      </SidebarContent>
      <SidebarFooter> 
        {connected ? (
          <WalletButton className="bg-gradient-to-r from-purple-500 to-pink-500 hover:from-purple-600 hover:to-pink-600 text-white font-semibold px-6 py-2 shadow-lg transition-all duration-200 hover:scale-105 hover:shadow-xl" />
        ) : (
          <WalletButton className="bg-gradient-to-r from-purple-500 to-pink-500 hover:from-purple-600 hover:to-pink-600 text-white font-semibold px-6 py-2 shadow-lg transition-all duration-200 hover:scale-105 hover:shadow-xl">
            <Wallet className="h-4 w-4 mr-2" />
            Connect Wallet
          </WalletButton>
        )}
      </SidebarFooter>
    </Sidebar>
  )
}
