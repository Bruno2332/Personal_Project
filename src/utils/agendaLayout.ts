import type { AgendaEvent } from "./agenda"
import {
    START_HOUR,
    END_HOUR,
    HOUR_HEIGHT,
    timeToMinutes,
} from "./agenda"

export type AgendaGroupLayout = {
    key: string
    events: AgendaEvent[]
    start: number
    end: number
    top: number
    height: number
    column: number
    columns: number
}

type EventGroup = {
    key: string
    events: AgendaEvent[]
    start: number
    end: number
    column: number
}

export default function calculateAgendaLayout(
    events: AgendaEvent[],
): AgendaGroupLayout[] {
    const groupsByStart = new Map<number, EventGroup>()

    for (const event of events) {
        const start = timeToMinutes(event.raw.start_time)
        const end = timeToMinutes(event.raw.end_time)

        if (
            !Number.isFinite(start) ||
            !Number.isFinite(end) ||
            end <= start ||
            end <= START_HOUR * 60 ||
            start >= END_HOUR * 60
        ) {
            continue
        }

        const existing = groupsByStart.get(start)

        if (existing) {
            existing.events.push(event)
            existing.end = Math.max(existing.end, end)
        } else {
            groupsByStart.set(start, {
                key: String(start),
                events: [event],
                start,
                end,
                column: 0,
            })
        }
    }

    const groups = [...groupsByStart.values()].sort(
        (a, b) => a.start - b.start,
    )

    const result: AgendaGroupLayout[] = []

    let overlappingGroups: EventGroup[] = []
    let columnEnds: number[] = []
    let latestEnd = -Infinity

    function finishGroup() {
        if (overlappingGroups.length === 0) return

        const columns = columnEnds.length

        for (const group of overlappingGroups) {
            const visibleStart = Math.max(
                group.start,
                START_HOUR * 60,
            )

            const visibleEnd = Math.min(
                group.end,
                END_HOUR * 60,
            )

            const top =
                ((visibleStart - START_HOUR * 60) / 60) *
                HOUR_HEIGHT

            const durationHeight =
                ((visibleEnd - visibleStart) / 60) *
                HOUR_HEIGHT

            const gap = Math.min(2, durationHeight / 4)

            result.push({
                ...group,
                top: top + gap / 2,
                height: durationHeight - gap,
                columns,
            })
        }

        overlappingGroups = []
        columnEnds = []
        latestEnd = -Infinity
    }

    for (const group of groups) {
        if (
            overlappingGroups.length > 0 &&
            group.start >= latestEnd
        ) {
            finishGroup()
        }

        const availableColumn = columnEnds.findIndex(
            (end) => end <= group.start,
        )

        group.column =
            availableColumn === -1
                ? columnEnds.length
                : availableColumn

        columnEnds[group.column] = group.end
        overlappingGroups.push(group)

        latestEnd = Math.max(latestEnd, group.end)
    }

    finishGroup()

    return result
}