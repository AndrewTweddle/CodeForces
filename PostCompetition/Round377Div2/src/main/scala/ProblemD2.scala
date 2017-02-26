import java.io._

import scala.io.Source
import scala.annotation.tailrec

object ProblemD2 {
  def main(args: Array[String]): Unit = processStdInOut()

  def processStdInOut(): Unit = {
    val src = Source.fromInputStream(System.in)
    processFromSource(src)
  }

  def processFromSource(src: Source): Unit = {
    try {
      val bw = new BufferedWriter(new OutputStreamWriter(System.out));
      try {
        val lines = src.getLines()
        processLines(lines, bw)
      } finally {
        bw.flush()
      }
    } finally {
      src.close();
    }
  }

  // Standard code above, custom code below

  // Wrap integers to benefit from compiler validation...
  case class Subject(subjectId: Int) extends AnyVal
  case class PrepTime(days: Int) extends AnyVal
  case class ExamDay(dayIndex: Int) extends AnyVal {
    def next(): ExamDay = ExamDay(dayIndex + 1)
  }

  def processLines(lines: Iterator[String], bw: BufferedWriter): Unit = {
    val line1 = lines.next()
    val strings = line1.split(" ")
    val n = strings(0).toInt
    val m = strings(1).toInt

    val line2 = lines.next()
    val d = line2.split(" ")
      .map(_.toInt)
      .map(di => if (di == 0) None else Some(Subject(di - 1)))  // zero-base the d's

    val line3 = lines.next()
    val a = line3.split(" ").map(pt => PrepTime(pt.toInt))

    val soln = solve(n, m, d, a)
    soln match {
      case Some(ExamDay(days)) => bw.write((days + 1).toString)  // Make d's one-based again
      case _ => bw.write("-1")
    }
    bw.newLine()
  }

  case class CumExamDay(day: ExamDay,	subject: Subject, subjectsToCover: Int,
                        daysFreeForStudy: Int, cumInvalidCount: Int)
  case class Accumulator(subjects: Set[Subject], schedule: List[CumExamDay])

  def solve(numDays: Int, numSubjects: Int, examByDay: Array[Option[Subject]], prepTimes: Array[PrepTime])
    : Option[ExamDay] = {

    def getPrepTime(subject: Subject): PrepTime = prepTimes(subject.subjectId)
    def getExamSubject(day: ExamDay): Option[Subject] = examByDay(day.dayIndex)

    def removeSubjectFromSchedule(subject: Subject, schedule: List[CumExamDay],
                                  freeDaysDelta: Int) : List[CumExamDay] = {

      @tailrec
      def merge(toFix: List[CumExamDay], fixed: List[CumExamDay], invalidCountDelta: Int): List[CumExamDay] = {
        toFix match {
          case Nil => fixed
          case hd :: tl => {
            val newDaysFree = hd.daysFreeForStudy + freeDaysDelta
            val newInvalidCountDelta =
              if (newDaysFree >= 0 && hd.daysFreeForStudy < 0) invalidCountDelta - 1 else invalidCountDelta
            val fixedHd = hd.copy(
              subjectsToCover = hd.subjectsToCover + 1,
              daysFreeForStudy = newDaysFree,
              cumInvalidCount = hd.cumInvalidCount + newInvalidCountDelta)
            merge(tl, fixedHd :: fixed, newInvalidCountDelta)
          }
        }
      }

      @tailrec
      def loop(scheduleToFixUp: List[CumExamDay], scheduleToProcess: List[CumExamDay]): List[CumExamDay] = {
        scheduleToProcess match {
          case h :: t if h.subject == subject => {
            val invalidCountDelta = if (h.daysFreeForStudy < 0) -1 else 0
            merge(scheduleToFixUp, t, invalidCountDelta)
          }
          case h :: t => loop(h :: scheduleToFixUp, t)
          case Nil => throw new NoSuchElementException("Subject not found")
        }
      }

      loop(Nil, schedule)
    }

    @tailrec
    def search(day: ExamDay, acc: Accumulator): Option[ExamDay] = {
      if (day.dayIndex >= numDays) None else {
        val subjectOpt = getExamSubject(day)
        val nextDay = day.next()
        subjectOpt match {
          case None => search(nextDay, acc)
          case Some(subject) => {
            val prepDaysForSubject = getPrepTime(subject).days
            if (acc.subjects.contains(subject)) {
              // Existing subject found:
              val schedWithoutSubject = removeSubjectFromSchedule(subject, acc.schedule, prepDaysForSubject + 1)
              if (schedWithoutSubject.isEmpty) {
                // Make a new schedule with the same subject, but a later day index...
                val daysFree = day.dayIndex - prepDaysForSubject
                val ced = CumExamDay(day, subject, numSubjects - 1, daysFree, 0)
                if (numSubjects == 1) Some(day)  // Solution found immediately
                else search(nextDay, Accumulator(Set(subject), List(ced)))
              } else {
                // Calculate the cumulative data for the subject...
                val h = schedWithoutSubject.head
                val daysFree = h.daysFreeForStudy + day.dayIndex - h.day.dayIndex - 1 - prepDaysForSubject
                val cumInvalidCount = if (daysFree >= 0) h.cumInvalidCount else h.cumInvalidCount + 1
                val subjectsToCover = h.subjectsToCover - 1
                if (cumInvalidCount == 0 && subjectsToCover == 0) Some(day)  // solution found immediately
                else {
                  // Move the subject to the end of the schedule...
                  val cumDay = CumExamDay(day, subject, subjectsToCover, daysFree, cumInvalidCount)
                  search(nextDay, acc.copy(schedule = cumDay :: schedWithoutSubject))
                }
              }
            } else {
              // New subject has been found...
              acc.schedule match {
                case Nil => {
                  // First subject in schedule...
                  val daysFree = day.dayIndex - prepDaysForSubject
                  if (daysFree >= 0) {
                    val ced = CumExamDay(day, subject, numSubjects - 1, daysFree, 0)
                    if (numSubjects == 1) Some(day)  // Solution found immediately
                    else search(nextDay, Accumulator(Set(subject), List(ced)))
                  } else {
                    // Ignore this entry... it can't be fitted in
                    search(nextDay, acc)
                  }
                }

                case h :: t => {
                  val daysFree = h.daysFreeForStudy + day.dayIndex - h.day.dayIndex - 1 - prepDaysForSubject
                  val cumInvalidCount = if (daysFree >= 0) h.cumInvalidCount else h.cumInvalidCount + 1
                  val subjectsToCover = h.subjectsToCover - 1
                  if (cumInvalidCount == 0 && subjectsToCover == 0) Some(day)
                  else {
                    val cumDay = CumExamDay(day, subject, subjectsToCover, daysFree, cumInvalidCount)
                    search(nextDay, Accumulator(acc.subjects + subject, cumDay :: acc.schedule))
                  }
                }
              }
            }
          }
        }
      }
    }

    search(ExamDay(0), Accumulator(Set.empty, List.empty))
  }
}
