import { useState } from 'react';
import { Input } from "@heroui/input";
import { Button } from "@heroui/button";
import { instance } from "@/env.tsx";
import { addToast, Form, NumberInput } from "@heroui/react";

export function SingleFileUploader() {
    const [file, setFile] = useState<File | null>(null);

    const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        if (e.target.files) {
            setFile(e.target.files[0]);
        }
    };

    const handleUpload = async (event) => {
        event.preventDefault();
        const formData = new FormData(event.target)

        return await instance.post('song', {
            bpm: formData.get("bpm"),
            name: formData.get("name"),
            file: formData.get("file")
        }, {
            headers: { "Content-Type": "multipart/form-data" }
        }).then((response) => {
            return addToast({
                title: "Mission réussi",
                description: response.data,
                variant: "bordered",
                color: "success",
            })
        }).catch((error) => {
            return addToast({
                title: "Echec de la mission",
                description: error,
                variant: "bordered",
                color: "danger",
            })
        })
    };

    return (
        <div className="bg-white p-6 rounded-lg shadow-md mb-6">
            <h2 className="text-xl font-bold mb-4">Ajouter une composition</h2>
            <Form onSubmit={handleUpload} className="flex flex-col gap-4">
                <div className="mb-2">
                    <Input
                        name="file"
                        type="file"
                        accept="audio/wav"
                        isRequired
                        onChange={handleFileChange}
                        className="w-full"
                    />
                </div>
                <div className="grid grid-cols-2 gap-4">
                    <NumberInput
                        name="bpm"
                        isRequired
                        label="BPM"
                        className="w-full"
                    />
                    <Input
                        name="name"
                        type="text"
                        isRequired
                        label="Nom"
                        className="w-full"
                    />
                </div>
                <Button
                    color="success"
                    type="submit"
                    className="mt-2 py-2"
                >
                    Sauvegarder la musique
                </Button>
            </Form>
        </div>
    );
}