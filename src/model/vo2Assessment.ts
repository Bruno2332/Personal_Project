
export type PerceivedExertion = 
    "Light" |
    "Moderate" | 
    "Intermediate" |
    "Intense" |
    "Maximal"


export type ExerciseHeartRateClassification =
    "Normal" |
    "Alert" |
    "Extreme" 

export type Vo2AssessmentDto = {
    id: number | null
    physical_assessment_id: number
    treadmill_speed_kmh: number | null
    heart_rate_bpm: number | null
    perceived_exertion: PerceivedExertion | null
    created_at: string
    updated_at: string
}

export type Vo2AssessmentCompleteDto = {
    vo2_assessment_dto: Vo2AssessmentDto
    fc_heart_percentage: number | null
    fc_heart_classification: ExerciseHeartRateClassification | null
} 