import React, { useState } from 'react';
import {Input} from "@heroui/input";
import {Button} from "@heroui/button";
import {instance} from "@/env.tsx";
import {Form} from "@heroui/react";

export function SingleFileUploader () {
    const [file, setFile] = useState<File | null>(null);

    const handleFileChange = (e: React.ChangeEvent<HTMLInputElement>) => {
        if (e.target.files) {
            setFile(e.target.files[0]);
        }
    };

    const handleUpload = async (event) => {
        event.preventDefault();
        const formData = new FormData(event.target)

        return await instance.post('song', {bpm: formData.get("bpm"), name: formData.get("name"), file: formData.get("file")}, {
            headers: {"Content-Type": "multipart/form-data", }
        })
    };

    return (
        <>
            <Form onSubmit={handleUpload}>
                <Input name="file" type={"file"} isRequired onChange={handleFileChange} />
                <Input name="bpm" type={"number"} isRequired label={"BPM"}/>
                <Input name="name" type={"text"} isRequired label={"Nom"}/>
                <Button
                    type={"submit"}
                    className="submit"
                >Upload a file</Button>
            </Form>
        </>
    );
}