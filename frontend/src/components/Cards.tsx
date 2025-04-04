import { Card, CardHeader, CardBody, CardFooter, Divider, Image } from "@heroui/react";
import { StarRating } from './StarRating';

export function Cards({ song }) {
    const handleRating = (rating) => {
        console.log('Note sélectionnée :', rating);
    };

    const formatDuration = () => {
        console.log(song.duration)
        const minutes = Math.floor(song.duration / 60);
        return `${minutes} minutes`;
    };

    return (
        <Card className="hover:shadow-lg transition-shadow duration-300 overflow-hidden h-64 w-full max-w-xs">
            <CardHeader className="flex gap-3 items-center pb-2 pt-3">
                <div className="bg-gradient-to-r from-blue-500 to-purple-500 rounded-md p-1 flex-shrink-0">
                    <Image
                        alt="Pochette du morceau"
                        height={40}
                        width={40}
                        radius="sm"
                        src="https://avatars.githubusercontent.com/u/86160567?s=200&v=4"
                        className="object-cover"
                    />
                </div>
                <div className="flex flex-col min-w-0">
                    <p className="text-base font-semibold line-clamp-1">{song.name || "Sans titre"}</p>
                    <p className="text-xs text-gray-500">Ajouté le {new Date(song.timestamp || Date.now()).toLocaleDateString()}</p>
                </div>
            </CardHeader>
            <Divider />
            <CardBody className="py-3">
                <div className="grid grid-cols-2 gap-1 text-xs">
                    <div className="flex items-center gap-1">
                        <p>Durée: {formatDuration()}</p>
                    </div>
                    <div className="flex items-center gap-1">
                        <p>Tempo: {song.bpm || "N/A"} bpm</p>
                    </div>
                    {song.path &&
                        <div className="flex items-center gap-1">
                            <audio controls>
                                <source src={song.path} type="audio/wav"/>
                                Your browser does not support the audio element.
                            </audio>
                        </div>
                    }
                    {/*<div className="flex items-center gap-1">
                        <audio controls>
                            <source src="../../Chaussette.wav" type="audio/wav"/>
                            Your browser does not support the audio element.
                        </audio>
                    </div>*/}
                </div>
            </CardBody>
            <Divider />
            <CardFooter className="flex justify-between items-center py-2">
                <StarRating
                    totalStars={5}
                    initialRating={song.rating}
                    onRatingChange={handleRating}
                />
            </CardFooter>
        </Card>
    );
}