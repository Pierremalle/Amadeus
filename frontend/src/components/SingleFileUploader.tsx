import { useState } from 'react';
import { Input } from "@heroui/input";
import { Button } from "@heroui/button";
import { instance } from "@/env.tsx";
import { addToast, Form, NumberInput } from "@heroui/react";

export function SingleFileUploader({ onUploadSuccess }) {
    const [file, setFile] = useState<File | null>(null);
    const [isLoading, setIsLoading] = useState(false);
    const [fileName, setFileName] = useState("");

    const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        if (e.target.files && e.target.files[0]) {
            const selectedFile = e.target.files[0];
            setFile(selectedFile);
            setFileName(selectedFile.name);
        }
    };

    const handleUpload = async (event) => {
        event.preventDefault();
        setIsLoading(true);
        const formData = new FormData(event.target);

        try {
            const response = await instance.post('song', {
                bpm: formData.get("bpm"),
                name: formData.get("name"),
                file: formData.get("file")
            }, {
                headers: { "Content-Type": "multipart/form-data" }
            });

            addToast({
                title: "Mission réussie",
                description: "Votre composition a été enregistrée avec succès.",
                variant: "bordered",
                color: "success",
            });

            event.target.reset();
            setFile(null);
            setFileName("");

            if (onUploadSuccess) onUploadSuccess();

        } catch (error) {
            addToast({
                title: "Échec de la mission",
                description: error.message || "Une erreur s'est produite lors de l'enregistrement.",
                variant: "bordered",
                color: "danger",
            });
        } finally {
            setIsLoading(false);
        }
    };

    return (
        <div className="bg-white p-6 rounded-lg shadow-sm">
            <Form onSubmit={handleUpload} className="flex flex-col gap-4">
                <div className="mb-2">
                    <label className="block text-sm font-medium text-gray-700 mb-1">
                        Fichier audio (WAV)
                    </label>
                    <div className="flex items-center gap-2">
                        <div className="relative flex-grow">
                            <Input
                                name="file"
                                type="file"
                                accept="audio/wav"
                                isRequired
                                onChange={handleFileChange}
                                className="w-full"
                                aria-label="Sélectionner un fichier audio"
                            />
                        </div>
                        {fileName && (
                            <div className="text-xs text-gray-500 truncate max-w-xs">
                                {fileName}
                            </div>
                        )}
                    </div>
                </div>

                <div className="grid grid-cols-1 sm:grid-cols-2 gap-4">
                    <div>
                        <NumberInput
                            name="bpm"
                            isRequired
                            label="BPM"
                            min={40}
                            max={300}
                            defaultValue={120}
                            className="w-full"
                        />
                    </div>
                    <div>
                        <Input
                            name="name"
                            type="text"
                            isRequired
                            label="Nom de la composition"
                            className="w-full"
                        />
                    </div>
                </div>

                <Button
                    color="success"
                    type="submit"
                    className="mt-2 py-2"
                    isLoading={isLoading}
                    startContent={!isLoading}
                >
                    {isLoading ? "Enregistrement en cours..." : "Sauvegarder la musique"}
                </Button>
            </Form>
        </div>
    );
}