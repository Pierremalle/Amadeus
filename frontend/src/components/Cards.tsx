import {Card, CardHeader, CardBody, CardFooter, Divider, Image} from "@heroui/react";
import { StarRating } from './StarRating'

export function Cards({song}) {
    const handleRating = (rating) => {
        console.log('Note sélectionnée :', rating);
        // Vous pouvez envoyer cette note à votre backend, par exemple
    };

    return (
        <Card className="m-12 mt-4">
            <CardHeader className="flex gap-3">
                <Image
                    alt="heroui logo"
                    height={40}
                    radius="sm"
                    src="https://avatars.githubusercontent.com/u/86160567?s=200&v=4"
                    width={40}
                />
                <div className="flex flex-col">
                    <p className="text-md">HeroUI</p>
                </div>
            </CardHeader>
            <Divider />
            <CardBody>
                <p>{song.name}</p>
                <p>Durée: 2:30</p>
                <p>Tempo: {song.bpm} bpm</p>
            </CardBody>
            <CardFooter className={"justify-end"}>
                <StarRating
                    totalStars={5}
                    onRatingChange={handleRating}
                />
            </CardFooter>
        </Card>
    );
}