import { useState } from 'react';
import { Star } from 'lucide-react';

export const StarRating = ({ totalStars = 5, onRatingChange }) => {
    const [rating, setRating] = useState(0);
    const [hover, setHover] = useState(0);

    const handleRatingChange = (currentRating) => {
        setRating(currentRating);
        if (onRatingChange) {
            onRatingChange(currentRating);
        }
    };

    return (
        <div className="flex">
            {[...Array(totalStars)].map((_, index) => {
                const currentRating = index + 1;
                return (
                    <Star
                        key={index}
                        size={24}
                        className={`cursor-pointer ${
                            currentRating <= (hover || rating)
                                ? 'text-yellow-500 fill-yellow-500'
                                : 'text-gray-300'
                        }`}
                        onMouseEnter={() => setHover(currentRating)}
                        onMouseLeave={() => setHover(0)}
                        onClick={() => handleRatingChange(currentRating)}
                    />
                );
            })}
            <span className="ml-2 text-gray-700">{rating} / {totalStars}</span>
        </div>
    );
};