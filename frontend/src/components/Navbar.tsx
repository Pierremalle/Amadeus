import {
    Navbar,
    NavbarBrand,
    NavbarContent,
    NavbarItem,
    NavbarMenuToggle,
    NavbarMenu,
    NavbarMenuItem,
    Dropdown,
    DropdownTrigger,
    Avatar,
    DropdownMenu,
    DropdownItem,
    User
} from "@heroui/react";
import { Link, useLocation } from "react-router-dom";
import { useState } from "react";

export const CustomLogo = () => {
    return (
        <img
            src="../../favicon.ico"
            alt="Amadeus"
            className="h-9 w-auto object-contain"
        />
    );
};

export function NavBar() {
    const [isMenuOpen, setIsMenuOpen] = useState(false);
    const location = useLocation();

    const navLinks = [
        { name: "Accueil", path: "/" },
        { name: "Compositions", path: "/" },
        { name: "Mes compositions", path: "/my_compositions" }
    ];

    return (
        <Navbar
            onMenuOpenChange={setIsMenuOpen}
            className="shadow-sm bg-white"
            maxWidth="full"
        >
            <NavbarContent className="sm:hidden" justify="start">
                <NavbarMenuToggle
                    aria-label={isMenuOpen ? "Fermer le menu" : "Ouvrir le menu"}
                    className="sm:hidden"
                />
            </NavbarContent>

            <NavbarBrand>
                <CustomLogo />
                <p className="font-bold text-inherit text-lg tracking-tight">Amadeus</p>
            </NavbarBrand>

            <NavbarContent className="hidden sm:flex gap-6" justify="center">
                {navLinks.map((link) => (
                    <NavbarItem
                        key={link.path}
                        isActive={location.pathname === link.path}
                    >
                        <Link
                            to={link.path}
                            className={`text-base font-medium transition-colors hover:text-primary ${
                                location.pathname === link.path
                                    ? "text-primary font-semibold"
                                    : "text-gray-700"
                            }`}
                        >
                            {link.name}
                        </Link>
                    </NavbarItem>
                ))}
            </NavbarContent>

            <NavbarContent as="div" justify="end">
                <Dropdown placement="bottom-end">
                    <DropdownTrigger>
                        <Avatar
                            isBordered
                            as="button"
                            className="transition-transform hover:scale-105"
                            color="secondary"
                            name="Jason Hughes"
                            size="sm"
                            src="https://i.pravatar.cc/150?u=a042581f4e29026704d"
                        />
                    </DropdownTrigger>
                    <DropdownMenu
                        aria-label="Profile Actions"
                        variant="flat"
                        className="w-64"
                    >
                        <DropdownItem key="profile" className="h-14 gap-2">
                            <User
                                avatarProps={{
                                    src: "https://i.pravatar.cc/150?u=a04258114e29026702d",
                                    size: "md",
                                }}
                                description="Développeur"
                                name="Ash"
                                className="py-2"
                            />
                        </DropdownItem>
                        <DropdownItem key="profil" className="text-gray-700 py-2">
                            <Link to="/profil" className="flex items-center gap-2">
                                Mon profil
                            </Link>
                        </DropdownItem>
                        <DropdownItem key="settings" className="text-gray-700 py-2">
                            <Link to="/my_compositions" className="flex items-center gap-2">
                                Mes compositions
                            </Link>
                        </DropdownItem>
                        <DropdownItem key="logout" color="danger" className="py-2">
                            <span className="flex items-center gap-2">
                                Se déconnecter
                            </span>
                        </DropdownItem>
                    </DropdownMenu>
                </Dropdown>
            </NavbarContent>

            <NavbarMenu>
                {navLinks.map((link) => (
                    <NavbarMenuItem key={link.path}>
                        <Link
                            to={link.path}
                            className={`text-base py-2 ${
                                location.pathname === link.path
                                    ? "text-primary font-medium"
                                    : "text-gray-700"
                            }`}
                            onClick={() => setIsMenuOpen(false)}
                        >
                            {link.name}
                        </Link>
                    </NavbarMenuItem>
                ))}
            </NavbarMenu>
        </Navbar>
    );
}