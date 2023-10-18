//
// Created by utente on 07/05/2023.
//
#include <stdio.h>
#include <stdlib.h>
#include <string.h>

typedef struct {
    int type;
    float val;
    long timestamp;
} ValueStruct;

typedef struct {
    int type;       //4 B
    float val[10];  //40 B
    long timestamp; //4 B
} MValueStruct;

typedef struct {
    int type;
    char message[21]; // stringa null terminated lunga max 20
} MessageStruct;

typedef struct {
    int type;
    union {
        ValueStruct val;
        MValueStruct mvals;
        MessageStruct messages;
    };
} ExportData;

void export(ExportData *data, int n, FILE *fp){
    int ret=fwrite(data,sizeof(ExportData),n,fp);
    fflush(fp);
    printf("ret: %d %llu",ret,sizeof(ExportData));
    if(ret!=n){
        printf("fwrite failed!");
    }
}


int main(){
    FILE* fp;
    ExportData data[100];
    ValueStruct valueTemp;
    MValueStruct mValueTemp;
    MessageStruct msgTemp;
    int i;

    printf("int: %d long: %d ValueStruct: %d",sizeof(int),sizeof(long),sizeof(ValueStruct));

    //setbuf(stdout,0);

    fp=fopen("../data.txt","wb");

    if(fp==NULL){
        printf("can't open file! \n");
        exit(1);
    }

    valueTemp.timestamp=200;
    valueTemp.type=11;
    valueTemp.val=150.26;

    for (i=0;i<33;i++) {
        data[i].type=0;
        data[i].val=valueTemp;
    }

    mValueTemp.timestamp=200;
    mValueTemp.type=1;
    for(int j=0;j<10;j++){
        mValueTemp.val[j]=1200.834;
    }


    for (i=33;i<66;i++) {
        data[i].type=1;
        data[i].mvals=mValueTemp;
    }


    msgTemp.type=2;
    strcpy(msgTemp.message," Ciao mamma! !!!!!!!");


    for (i=66;i<100;i++) {
        data[i].type=2;
        data[i].messages=msgTemp;
    }

    export(data,100,fp);
    fclose(fp);
    return 0;
}

