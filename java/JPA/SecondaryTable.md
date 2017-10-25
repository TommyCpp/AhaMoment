# Description
Sometimes we want to add some column on a existing table. Or we want to fetch one table to many.

We can use @SecondaryTable to inject field from other table into target table

Note that It only suitable for the one-to-one relationship.

## Environment
* JPA2
* Hibernate
# Example
## Database
Given such relationship.
![](https://github.com/TommyCpp/AhaMoment/blob/master/copy.png)

We can construct the following Entity

```java
@Entity
@Table(name = "audio_copy") 
@SecondaryTable(name = "copy", pkJoinColumns = {@PrimaryKeyJoinColumn(name = "id", referencedColumnName = "copy_id")})
public class TestAudioCopy {
    private Long copyId;
    private Audio audio;
    private Integer status;
    private Timestamp updatedDate;
    private Timestamp createdDate;

    @ManyToOne
    @JoinColumn(name = "isrc", table = "audio_copy", referencedColumnName = "isrc")
    public Audio getAudio() {
        return audio;
    }

    public void setAudio(Audio audio) {
        this.audio = audio;
    }

    @Id
    @Column(name = "copy_id", table = "audio_copy")
    public Long getCopyId() {
        return copyId;
    }

    public void setCopyId(Long copyId) {
        this.copyId = copyId;
    }


    @Basic
    @Column(name = "status", nullable = false, table = "copy")
    public Integer getStatus() {
        return status;
    }

    public void setStatus(Integer status) {
        this.status = status;
    }

    @Basic
    @Column(name = "created_date", nullable = true, table = "copy")
    public Timestamp getCreatedDate() {
        return createdDate;
    }

    public void setCreatedDate(Timestamp createdDate) {
        this.createdDate = createdDate;
    }

    @Basic
    @Column(name = "updated_date", nullable = true, table = "copy")
    public Timestamp getUpdatedDate() {
        return updatedDate;
    }

    public void setUpdatedDate(Timestamp updatedDate) {
        this.updatedDate = updatedDate;
    }
}
```

```java
@Entity
@Table(name = "audio")
public class Audio{
    ....
    @OneToMany(mappedBy = "audio")
    public List<TestAudioCopy> getCopies() {

        return copies;
    }

    public void setCopies(List<TestAudioCopy> copies) {
        this.copies = copies;
    }
    ....
}
```

**We inject the field of the `copy` table into `audio_copy` table**

* In `TestAudioCopy`, we can use the field in `copy` just as its own field.
* But we need to specify the `table` attribute in the `@Column` annotation.
* In order to run the application, the `copy_id` must be include in one **field** in `TestAudioCopy` or its supermapper or secondary table.

# Reference
https://www.concretepage.com/hibernate/secondarytables_hibernate_annotation
