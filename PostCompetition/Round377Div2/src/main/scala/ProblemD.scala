import java.io._

import scala.io.Source
import scala.annotation.tailrec
import scala.collection.mutable.ArrayBuffer

object ProblemD {
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
  case class ExamDay(dayIndex: Int) extends AnyVal with Ordered[ExamDay] {
    def compare(that: ExamDay) = this.dayIndex - that.dayIndex
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
      case Some(ExamDay(days)) => bw.write((days + 1).toString)  // It was zero-based
      case _ => bw.write("-1")
    }
    bw.newLine()
  }

  case class SubjectExamDay(subject: Subject, day: ExamDay)

  def solve(n: Int, m: Int, d: Array[Option[Subject]], a: Array[PrepTime]): Option[ExamDay] = {
    def getPrepTime(subject: Subject): PrepTime = a(subject.subjectId)
    def getExamSubject(day: ExamDay): Option[Subject] = d(day.dayIndex)

    val subjectExamDays = Array.fill[ArrayBuffer[SubjectExamDay]](m)(ArrayBuffer[SubjectExamDay]())
    d.zipWithIndex
      .collect { case (Some(prepTm), ix) => (prepTm, ExamDay(ix)) }
      .map(di => SubjectExamDay(di._1, di._2))
      .foreach( sed => subjectExamDays(sed.subject.subjectId) += sed )

    val candidateDays = Array.fill(m)(0)
    val initialCandidates = (0 until m).map(index => subjectExamDays(index)(candidateDays(index)))
    val initialDay = initialCandidates.map(_.day).max

    @tailrec
    def findSolution(currDay: ExamDay, currCandidates: IndexedSeq[SubjectExamDay]): Option[ExamDay] = {
      def isValidSolution(): Boolean = {
        val orderedCandidates = currCandidates.sortBy(_.day.dayIndex)
        val cumDays = orderedCandidates
          .map(cand => getPrepTime(cand.subject).days + 1)
          .scanLeft(0)(_ + _).drop(1)  // The first element is always zero, so ignore it
        orderedCandidates.zip(cumDays).forall(candCum => candCum._2 <= (candCum._1.day.dayIndex + 1))
      }
      @tailrec
      def advanceDayIndex(fromDay: ExamDay): Option[ExamDay] = {
        val nextDayIndex = fromDay.dayIndex + 1
        if (nextDayIndex == n) None
        else {
          val nextDay = ExamDay(nextDayIndex)
          if (getExamSubject(nextDay).isDefined) Some(nextDay)
          else advanceDayIndex(nextDay)
        }
      }

      if (isValidSolution()) Some(currDay)
      else {
        val newDayOpt = advanceDayIndex(currDay)
        newDayOpt match {
          case None => None
          case Some(newDay) => {
            val subjectOfNewDay = getExamSubject(newDay).get
            candidateDays(subjectOfNewDay.subjectId) += 1
            val newCandidates = (0 until m).map(index => subjectExamDays(index)(candidateDays(index)))
            findSolution(newDay, newCandidates)
          }
        }
      }
    }

    findSolution(initialDay, initialCandidates)
  }
}
